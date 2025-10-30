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
args = parser.parse_args()
print(args.path)

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
    count = 0
    total_files = len([file for file in os.listdir(path_to_json) if file.endswith('.json')])

    for file_name in [file for file in os.listdir(path_to_json) if file.endswith('.json')]:
        file_name_splitted = file_name.rsplit("_", 2)
        original_file_name = file_name_splitted[0] + '_' + str(count).zfill(12) + '_' + file_name_splitted[2]

        with open(os.path.join(path_to_json, original_file_name)) as json_file:
            data = json.load(json_file)
            count += 1
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

# Process each subject folder
def process_subjects_json():
    discrepancy_subjects = []

    for subject_name in os.listdir(INPUT_ROOT):
        subject_folder = os.path.join(INPUT_ROOT, subject_name)
        if os.path.isdir(subject_folder):
            # Calculate keypoints data and check for discrepancies
            keypoints_data, total_files, total_frames_processed = calculate_keypoints(subject_folder)

            if total_files != total_frames_processed:
                discrepancy_subjects.append(subject_name)
                print(
                    f"Discrepancy found for {subject_name}: Total files = {total_files}, Frames processed = {total_frames_processed}")

            # Calculate angles
            hip_flex_ext = calculate_hip_flexion_extension(keypoints_data)
            knee_flex_ext = calculate_knee_flexion_extension(keypoints_data)
            knee_abd_add = calculate_knee_abduction_adduction(keypoints_data)
            hip_abd_add = calculate_hip_abduction_adduction(keypoints_data)

            # Create a DataFrame with all angles, using your specified column names
            df_results = pd.DataFrame({
                'Knee Flexion/Extension Left': knee_flex_ext[0],
                'Knee Flexion/Extension Right': knee_flex_ext[1],
                'Hip Flexion/Extension Left': hip_flex_ext[0],
                'Hip Flexion/Extension Right': hip_flex_ext[1],
                'Knee Abduction/Adduction Left': knee_abd_add[0],
                'Knee Abduction/Adduction Right': knee_abd_add[1],
                'Hip Abduction/Adduction Left': hip_abd_add[0],
                'Hip Abduction/Adduction Right': hip_abd_add[1]
            })

            # Select columns based on '_F_' or '_S_' in the file name
            if '_F_' in subject_name:
                df_results_filtered = df_results[
                    ['Knee Abduction/Adduction Left', 'Knee Abduction/Adduction Right',
                     'Hip Abduction/Adduction Left', 'Hip Abduction/Adduction Right']
                ]
            elif '_S_' in subject_name:
                df_results_filtered = df_results[
                    ['Knee Flexion/Extension Left', 'Knee Flexion/Extension Right',
                     'Hip Flexion/Extension Left', 'Hip Flexion/Extension Right']
                ]
            else:
                continue  # Skip if the filename doesn't contain '_F_' or '_S_'

            # Save the filtered DataFrame to CSV
            df_results_filtered.to_csv(os.path.join(OUTPUT_FOLDER, f'{subject_name}.csv'), index=False)
            print(f"Saved angles for {subject_name} to {OUTPUT_FOLDER}/{subject_name}.csv")

    if discrepancy_subjects:
        print("\nSubjects with discrepancies in input and output frame count:", discrepancy_subjects)

# Process each subject folder
def process_subjects_gait():
    discrepancy_subjects = []

    for subject_name in os.listdir(INPUT_ROOT):
        subject_folder = os.path.join(INPUT_ROOT, subject_name)
        if os.path.isdir(subject_folder):
            # Calculate keypoints data and check for discrepancies
            keypoints_data, total_files, total_frames_processed = calculate_keypoints(subject_folder)

            if total_files != total_frames_processed:
                discrepancy_subjects.append(subject_name)
                print(
                    f"Discrepancy found for {subject_name}: Total files = {total_files}, Frames processed = {total_frames_processed}")

            # Calculate angles
            hip_flex_ext = calculate_hip_flexion_extension(keypoints_data)
            knee_flex_ext = calculate_knee_flexion_extension(keypoints_data)
            knee_abd_add = calculate_knee_abduction_adduction(keypoints_data)
            hip_abd_add = calculate_hip_abduction_adduction(keypoints_data)

            # Create a DataFrame with all angles, using your specified column names
            df_results = pd.DataFrame({
                'Knee Flexion/Extension Left': knee_flex_ext[0],
                'Knee Flexion/Extension Right': knee_flex_ext[1],
                'Hip Flexion/Extension Left': hip_flex_ext[0],
                'Hip Flexion/Extension Right': hip_flex_ext[1],
                'Knee Abduction/Adduction Left': knee_abd_add[0],
                'Knee Abduction/Adduction Right': knee_abd_add[1],
                'Hip Abduction/Adduction Left': hip_abd_add[0],
                'Hip Abduction/Adduction Right': hip_abd_add[1]
            })

            # Select columns based on '_F_' or '_S_' in the file name
            if '_F_' in subject_name:
                df_results_filtered = df_results[
                    ['Knee Abduction/Adduction Left', 'Knee Abduction/Adduction Right',
                     'Hip Abduction/Adduction Left', 'Hip Abduction/Adduction Right']
                ]
            elif '_S_' in subject_name:
                df_results_filtered = df_results[
                    ['Knee Flexion/Extension Left', 'Knee Flexion/Extension Right',
                     'Hip Flexion/Extension Left', 'Hip Flexion/Extension Right']
                ]
            else:
                continue  # Skip if the filename doesn't contain '_F_' or '_S_'

            # Save the filtered DataFrame to CSV
            df_results_filtered.to_csv(os.path.join(OUTPUT_FOLDER, f'{subject_name}.csv'), index=False)
            print(f"Saved angles for {subject_name} to {OUTPUT_FOLDER}/{subject_name}.csv")

    if discrepancy_subjects:
        print("\nSubjects with discrepancies:", discrepancy_subjects)

# Run the process
process_subjects_json()

# Run the process
process_subjects_gait()


# File paths
csv_dir = args.path + '/Gait_parameters'
output_dir_selected = args.path + '/Selected_one_gait_CSV'

# Ensure the output directories exist
os.makedirs(output_dir_selected, exist_ok=True)

# Define column selections for front and side videos
columns_front = ['Knee Abduction/Adduction Left', 'Knee Abduction/Adduction Right',
                 'Hip Abduction/Adduction Left', 'Hip Abduction/Adduction Right']
columns_side = ['Knee Flexion/Extension Left', 'Knee Flexion/Extension Right',
                'Hip Flexion/Extension Left', 'Hip Flexion/Extension Right']

# Target number of rows for interpolation
target_rows = 30

# Loop through all files in the specified directory
for subject_name in os.listdir(csv_dir):
    if not subject_name.endswith('.csv'):
        continue  # Skip non-CSV files

    print(f"\nProcessing subject: {subject_name}")

    # Construct the file path for the current subject's CSV
    csv_file = os.path.join(csv_dir, subject_name)

    # Determine if it's a front or side video based on the filename
    if '_F_' in subject_name:
        columns = columns_front
        video_type = 'Front'
    elif '_S_' in subject_name:
        columns = columns_side
        video_type = 'Side'
    else:
        print(f"Error: Invalid filename format for subject {subject_name}")
        continue

    if os.path.isfile(csv_file):
        # Load the corresponding CSV file
        df = pd.read_csv(csv_file)
        print(f"Loaded {video_type} CSV file for subject: {subject_name}, shape: {df.shape}")

        # Get the row count of the DataFrame
        row_count = df.shape[0]

        # Calculate starting indices for the four segments
        segment_starts = [int(row_count * i / 5) for i in range(1, 5)]
        segment_length = 30  # Number of frames to select per segment

        interpolated_segments = []

        # Process each segment
        for i, start in enumerate(segment_starts, start=1):
            end = start + segment_length
            if end > row_count:
                print(f"Segment {i} exceeds row count, skipping...")
                continue

            segment = df.iloc[start:end]

            # Ensure only selected columns are included
            interpolated_segment = segment[columns].copy()
            interpolated_segments.append(interpolated_segment)
            print(f"Segment {i}: Selected {segment_length} rows from index {start} to {end - 1}")

        # Concatenate all segments for this subject
        final_df = pd.concat(interpolated_segments, ignore_index=True)

        # Save the final interpolated data for the subject
        selected_output_path = os.path.join(output_dir_selected, f'{subject_name}_interpolated_one_gait_per_segment.csv')
        final_df.to_csv(selected_output_path, index=False)
        print(f"Saved interpolated segments to: {selected_output_path}")

    else:
        print(f"File not found for subject: {subject_name}")

print("Processing complete!")

# Input and output folder paths
input_folder = args.path + '/Selected_one_gait_CSV'
output_folder = args.path + '/Combined_front_side_columns'

# Ensure the output directory exists
os.makedirs(output_folder, exist_ok=True)
print(f"Output directory ensured: {output_folder}")

# Dictionary to group files by their common part after `Su_XXX_F_` or `Su_XXX_S_`
file_groups = {}

# Group files by unique identifiers (common 'rest of the file' part)
print("Grouping files by shared parts...")
for filename in os.listdir(input_folder):
    if filename.endswith(".csv"):
        parts = filename.split('_', maxsplit=4)  # Split into meaningful parts
        if len(parts) > 4 and (parts[2] == 'F' or parts[2] == 'S'):
            key = '_'.join(parts[:2] + parts[4:])  # Combine Su_XXX and {rest_of_file}
            if key not in file_groups:
                file_groups[key] = {}
            file_groups[key][parts[2]] = filename  # Store files based on 'F' or 'S'

# Debugging: Show grouped files
print("Grouped files:")
for group_key, files in file_groups.items():
    print(f"{group_key}: {files}")

# Process each group
for group_key, files in file_groups.items():
    if 'F' in files and 'S' in files:  # Ensure both F and S files exist
        print(f"\nProcessing group: {group_key}")

        # Read the two files
        file1_path = os.path.join(input_folder, files['F'])
        file2_path = os.path.join(input_folder, files['S'])
        print(f"Reading files:\n - {file1_path}\n - {file2_path}")
        df1 = pd.read_csv(file1_path)
        df2 = pd.read_csv(file2_path)

        # Combine columns
        combined_df = pd.concat([df1, df2], axis=1)
        print("Columns combined.")

        # Add ASD column
        asd_value = 0 if group_key.startswith('Su_1') else 1
        combined_df['ASD'] = asd_value
        print(f"ASD column added with value: {asd_value}")

        # Generate output filename
        prefix = group_key.split('_')[0]  # Get the Su_XXX prefix
        shared_name = group_key.split('_', maxsplit=1)[1]  # Rest of the file part
        output_filename = f"{prefix}_combined_{shared_name}"
        output_path = os.path.join(output_folder, output_filename)
        print(f"Output filename: {output_filename}")

        # Save the combined DataFrame
        combined_df.to_csv(output_path, index=False)
        print(f"File saved to: {output_path}")
    else:
        print(f"Skipping group {group_key}, missing 'F' or 'S' file.")

print("\nProcessing complete.")

import os
import numpy as np
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model

# --- Load the trained model ---
model_path = args.path + '/Model/ASD_Classifier_Model.h5'  # Replace with your actual model file path
model = load_model(model_path)  # Load the saved model

# --- Folder containing the CSV files ---
input_folder = args.path + '/Combined_front_side_columns'

# --- Function to validate CSV files ---
def validate_csv(file_path):
    """
    Validates if a CSV file is properly formatted and non-empty.
    Returns the DataFrame if valid, otherwise returns None.
    """
    try:
        df = pd.read_csv(file_path)  # Attempt to load the CSV file
        if df.empty:  # Check if the file is empty
            raise ValueError("Empty CSV file.")
        return df
    except Exception as e:  # Catch errors in file reading
        print(f"[WARNING] Skipping file {file_path} due to error: {e}")
        return None

# List to store results
results = []

for filename in os.listdir(input_folder):
    if filename.endswith(".csv"):  # Process only CSV files
        file_path = os.path.join(input_folder, filename)  # Create the full file path
        df = validate_csv(file_path)  # Validate the file

        if df is not None:  # If valid, process the data
            data = df.iloc[:, :-1].values  # Extract all columns except the last
            scaler = MinMaxScaler()  # Initialize MinMaxScaler for normalization
            data = scaler.fit_transform(data)  # Normalize the data
            data_flat = data.reshape(1, -1)  # Flatten the data for prediction

            # Make prediction
            prediction = model.predict(data_flat)  # Get model prediction
            prediction_label = 'ASD' if prediction > 0.5 else 'No ASD'  # Threshold for binary classification

            # Append results to the list
            results.append({"File": filename, "Score": prediction, "Prediction": prediction_label})

            # Print the filename and prediction
            print(f"File: {filename}, Prediction: {prediction_label} ({prediction * 100}%)")

# Save results to a CSV file
output_dir = args.path + '/Results'
output_file = os.path.join(output_dir, "predictions.csv")
os.makedirs(output_dir, exist_ok=True)
individual_output = os.path.join(args.result_path, 'final_score')
with open(individual_output, 'w', encoding="utf-8") as f:
    f.write(str(results[0]["Score"][0][0]))
    f.close()

# Save the results to the specified path
results_df = pd.DataFrame(results)  # Convert results to a DataFrame
results_df.to_csv(output_file, index=False)  # Save DataFrame to CSV file

print(f"Results saved to {output_file}")
