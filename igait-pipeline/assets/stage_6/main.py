import os
import json
import math
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from sklearn.linear_model import LinearRegression
from sklearn.experimental import enable_iterative_imputer
from sklearn.impute import IterativeImputer
from scipy.interpolate import CubicSpline
import argparse

# Import paths
parser = argparse.ArgumentParser()
parser.add_argument("path")
parser.add_argument("result_path")
parser.add_argument("model_path", help="Path to the Model directory")
args = parser.parse_args()
print(f"Processing path: {args.path}")
print(f"Model path: {args.model_path}")

# Folder Paths
INPUT_ROOT = args.path + '/json'
OUTPUT_FOLDER = args.path + '/Gait_parameters/'

# Create output folder if it doesn't exist
os.makedirs(OUTPUT_FOLDER, exist_ok=True)

# Replace 0 values with NaN to handle missing data
def zero_to_nan(values):
    return [float('nan') if x == 0 else x for x in values]

# Fill missing values using Iterative Imputer with Linear Regression
def fill_missing_values(df, tolerance=5, last_missing_count=None):
    cols_with_missing = df.columns[df.isna().any()].tolist()
    current_missing_count = df.isna().sum().sum()
    if not cols_with_missing or (
            last_missing_count is not None and abs(last_missing_count - current_missing_count) <= tolerance):
        return df

    if set(cols_with_missing) == set(df.columns):
        imputer = IterativeImputer(estimator=LinearRegression(), random_state=0)
        imputed_data = imputer.fit_transform(df)
        return pd.DataFrame(imputed_data, columns=df.columns, index=df.index)

    predictors = df.drop(columns=cols_with_missing)
    for col in cols_with_missing:
        training_data = df.dropna(subset=[col])
        test_data = df[df[col].isnull()]
        X_train = training_data[predictors.columns]
        y_train = training_data[col]
        model = LinearRegression()
        model.fit(X_train, y_train)
        X_test = test_data[predictors.columns]
        predictions = model.predict(X_test)
        df.loc[df[col].isnull(), col] = predictions

    return fill_missing_values(df, tolerance=tolerance, last_missing_count=current_missing_count)

# Calculate angle using three points
def calculate_angle_between_points(p1, p2, p3):
    a = math.sqrt((p2[0] - p1[0]) ** 2 + (p2[1] - p1[1]) ** 2)
    b = math.sqrt((p2[0] - p3[0]) ** 2 + (p2[1] - p3[1]) ** 2)
    c = math.sqrt((p3[0] - p1[0]) ** 2 + (p3[1] - p1[1]) ** 2)
    angle = math.acos((a ** 2 + b ** 2 - c ** 2) / (2 * a * b))
    return math.degrees(angle)

# Calculate hip flexion/extension angle
def calc_hip_angle_S(hip, knee):
    if hip == [-1, -1] or knee == [-1, -1]:
        return None
    a = np.array(hip)
    b = np.array(knee)
    ab = b - a
    m_N = np.array([0, -1])
    cosine_angle = np.dot(ab, m_N) / (np.linalg.norm(ab) * np.linalg.norm(m_N))
    angle = np.arccos(cosine_angle)
    return np.degrees(angle).tolist()

# Calculate hip flexion/extension for all frames
def calculate_hip_flexion_extension(data):
    hip_ang_L = []
    hip_ang_R = []
    for coord in data:
        # Right
        RHip = [coord[18], coord[19]]
        RKnee = [coord[20], coord[21]]
        angle = calc_hip_angle_S(RHip, RKnee)
        hip_ang_R.append(180 - angle)

        # Left
        LHip = [coord[24], coord[25]]
        LKnee = [coord[26], coord[27]]
        angle = calc_hip_angle_S(LHip, LKnee)
        hip_ang_L.append(180 - angle)

    return [hip_ang_L, hip_ang_R]

# Calculate knee flexion/extension for all frames
def calculate_knee_flexion_extension(data):
    knee_flexion_extension_ang_L = []
    knee_flexion_extension_ang_R = []
    for coord in data:
        # Right
        RHip = [coord[18], coord[19]]
        RKnee = [coord[20], coord[21]]
        RAnkle = [coord[22], coord[23]]
        angle = 180 - calculate_angle_between_points(RHip, RKnee, RAnkle)
        knee_flexion_extension_ang_R.append(angle)

        # Left
        LHip = [coord[24], coord[25]]
        LKnee = [coord[26], coord[27]]
        LAnkle = [coord[28], coord[29]]
        angle = 180 - calculate_angle_between_points(LHip, LKnee, LAnkle)
        knee_flexion_extension_ang_L.append(angle)

    return [knee_flexion_extension_ang_L, knee_flexion_extension_ang_R]

# Calculate knee abduction/adduction for all frames
def calculate_knee_abduction_adduction(data):
    knee_abd_add_ang_L = []
    knee_abd_add_ang_R = []
    for coord in data:
        # Right
        RHip = [coord[18], coord[19]]
        RKnee = [coord[20], coord[21]]
        RAnkle = [coord[22], coord[23]]
        angle = calculate_angle_between_points(RHip, RKnee, RAnkle)
        knee_abd_add_ang_R.append(angle)

        # Left
        LHip = [coord[24], coord[25]]
        LKnee = [coord[26], coord[27]]
        LAnkle = [coord[28], coord[29]]
        angle = calculate_angle_between_points(LHip, LKnee, LAnkle)
        knee_abd_add_ang_L.append(angle)

    return [knee_abd_add_ang_L, knee_abd_add_ang_R]

# Calculate hip abduction/adduction for all frames
def calculate_hip_abduction_adduction(data):
    hip_abd_add_ang_L = []
    hip_abd_add_ang_R = []
    for coord in data:
        # Right
        RShoulder = [coord[6], coord[7]]
        RHip = [coord[18], coord[19]]
        RKnee = [coord[20], coord[21]]
        angle = calculate_angle_between_points(RShoulder, RHip, RKnee)
        hip_abd_add_ang_R.append(angle)

        # Left
        LShoulder = [coord[12], coord[13]]
        LHip = [coord[24], coord[25]]
        LKnee = [coord[26], coord[27]]
        angle = calculate_angle_between_points(LShoulder, LHip, LKnee)
        hip_abd_add_ang_L.append(angle)

    return [hip_abd_add_ang_L, hip_abd_add_ang_R]

# Process keypoints from JSON files
def calculate_keypoints(path_to_json):
    interpolated_coordinates = []
    
    # Get all JSON files and sort them
    json_files = sorted([f for f in os.listdir(path_to_json) if f.endswith('.json')])
    total_files = len(json_files)

    for file_name in json_files:
        file_path = os.path.join(path_to_json, file_name)
        
        with open(file_path) as json_file:
            data = json.load(json_file)
            if len(data["people"]):
                coordinates = data["people"][0]["pose_keypoints_2d"]
                coordinates = zero_to_nan(coordinates)

                coord = [coordinates[i:i + 2] for i in range(0, len(coordinates), 3)]
                interpolated_coordinates.append([coord[i][0] for i in range(25)] + [coord[i][1] for i in range(25)])

    df = pd.DataFrame(interpolated_coordinates, columns=[f'{joint}{axis}' for joint in [
        'Nose', 'Neck', 'RShoulder', 'RElbow', 'RWrist', 'LShoulder', 'LElbow', 'LWrist',
        'MHip', 'RHip', 'RKnee', 'RAnkle', 'LHip', 'LKnee', 'LAnkle', 'REye', 'LEye',
        'REar', 'LEar', 'LBigToe', 'LSmallToe', 'LHeel', 'RBigToe', 'RSmallToe', 'RHeel']
                                                         for axis in ['X', 'Y']])

    df = fill_missing_values(df)
    return df.values.tolist(), total_files, df.shape[0]

# Process front and side JSON directories
print("\n=== Processing JSON files ===")
front_json_dir = os.path.join(INPUT_ROOT, 'front')
side_json_dir = os.path.join(INPUT_ROOT, 'side')

# Check both directories exist
if not os.path.exists(front_json_dir):
    raise FileNotFoundError(f"Front JSON directory not found: {front_json_dir}")
if not os.path.exists(side_json_dir):
    raise FileNotFoundError(f"Side JSON directory not found: {side_json_dir}")

# Process front video (for abduction/adduction angles)
print(f"\nProcessing front video: {front_json_dir}")
keypoints_data_front, total_files_front, frames_processed_front = calculate_keypoints(front_json_dir)
print(f"Front: {total_files_front} files, {frames_processed_front} frames processed")

knee_abd_add = calculate_knee_abduction_adduction(keypoints_data_front)
hip_abd_add = calculate_hip_abduction_adduction(keypoints_data_front)

df_front = pd.DataFrame({
    'Knee Abduction/Adduction Left': knee_abd_add[0],
    'Knee Abduction/Adduction Right': knee_abd_add[1],
    'Hip Abduction/Adduction Left': hip_abd_add[0],
    'Hip Abduction/Adduction Right': hip_abd_add[1]
})

front_csv_path = os.path.join(OUTPUT_FOLDER, 'front_angles.csv')
df_front.to_csv(front_csv_path, index=False)
print(f"Saved front angles to: {front_csv_path}")

# Process side video (for flexion/extension angles)
print(f"\nProcessing side video: {side_json_dir}")
keypoints_data_side, total_files_side, frames_processed_side = calculate_keypoints(side_json_dir)
print(f"Side: {total_files_side} files, {frames_processed_side} frames processed")

hip_flex_ext = calculate_hip_flexion_extension(keypoints_data_side)
knee_flex_ext = calculate_knee_flexion_extension(keypoints_data_side)

df_side = pd.DataFrame({
    'Knee Flexion/Extension Left': knee_flex_ext[0],
    'Knee Flexion/Extension Right': knee_flex_ext[1],
    'Hip Flexion/Extension Left': hip_flex_ext[0],
    'Hip Flexion/Extension Right': hip_flex_ext[1]
})

side_csv_path = os.path.join(OUTPUT_FOLDER, 'side_angles.csv')
df_side.to_csv(side_csv_path, index=False)
print(f"Saved side angles to: {side_csv_path}")

# === Select one gait cycle from each segment ===
print("\n=== Selecting gait cycles ===")
output_dir_selected = args.path + '/Selected_one_gait_CSV'
os.makedirs(output_dir_selected, exist_ok=True)

target_rows = 30

def select_gait_segments(df, name):
    """Select 4 segments of 30 frames each from the dataframe"""
    row_count = df.shape[0]
    segment_starts = [int(row_count * i / 5) for i in range(1, 5)]
    segment_length = 30
    
    interpolated_segments = []
    for i, start in enumerate(segment_starts, start=1):
        end = start + segment_length
        if end > row_count:
            print(f"Segment {i} exceeds row count, skipping...")
            continue
        
        segment = df.iloc[start:end].copy()
        interpolated_segments.append(segment)
        print(f"{name} Segment {i}: Selected {segment_length} rows from index {start} to {end - 1}")
    
    return pd.concat(interpolated_segments, ignore_index=True)

# Process front and side
front_selected = select_gait_segments(df_front, "Front")
front_selected_path = os.path.join(output_dir_selected, 'front_interpolated.csv')
front_selected.to_csv(front_selected_path, index=False)
print(f"Saved front selected segments to: {front_selected_path}")

side_selected = select_gait_segments(df_side, "Side")
side_selected_path = os.path.join(output_dir_selected, 'side_interpolated.csv')
side_selected.to_csv(side_selected_path, index=False)
print(f"Saved side selected segments to: {side_selected_path}")

# === Combine front and side columns ===
print("\n=== Combining front and side data ===")
output_folder_combined = args.path + '/Combined_front_side_columns'
os.makedirs(output_folder_combined, exist_ok=True)

# Combine columns from front and side
combined_df = pd.concat([front_selected, side_selected], axis=1)
print(f"Combined shape: {combined_df.shape}")

# Save combined data
combined_path = os.path.join(output_folder_combined, 'combined_data.csv')
combined_df.to_csv(combined_path, index=False)
print(f"Saved combined data to: {combined_path}")

# === Make prediction ===
print("\n=== Making prediction ===")
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model

# Load the trained model from the provided model path
model_path = os.path.join(args.model_path, 'ASD_Classifier_Model.h5')
model = load_model(model_path)
print(f"Loaded model from: {model_path}")

# Prepare data for prediction
data = combined_df.values
scaler = MinMaxScaler()
data = scaler.fit_transform(data)
data_flat = data.reshape(1, -1)

# Make prediction
prediction = model.predict(data_flat)
prediction_score = prediction[0][0]
prediction_label = 'ASD' if prediction_score > 0.5 else 'No ASD'

print(f"\nPrediction: {prediction_label}")
print(f"Score: {prediction_score:.4f} ({prediction_score * 100:.2f}%)")

# Save results
output_dir_results = args.path + '/Results'
os.makedirs(output_dir_results, exist_ok=True)

# Save prediction score to final_score file
final_score_path = os.path.join(args.result_path, 'final_score')
with open(final_score_path, 'w', encoding="utf-8") as f:
    f.write(str(prediction_score))
print(f"\nSaved final score to: {final_score_path}")

# Save detailed results to CSV
results_df = pd.DataFrame([{
    "Score": prediction_score,
    "Prediction": prediction_label
}])
results_csv_path = os.path.join(output_dir_results, "predictions.csv")
results_df.to_csv(results_csv_path, index=False)
print(f"Saved detailed results to: {results_csv_path}")

print("\n=== Processing complete! ===")
