from time import sleep
from random import random
import sys
from os.path import exists, dirname, realpath

def main():
    front_path = sys.argv[1]
    side_path = sys.argv[2]

    if not exists(f"{dirname(realpath(__file__))}/queue/{front_path}"):
        print(f"File {dirname(realpath(__file__))}/queue/{front_path} does not exist!")
        return
    if not exists(f"{dirname(realpath(__file__))}/queue/{side_path}"):
        print(f"File {dirname(realpath(__file__))}/queue/{side_path} does not exist!")
        return

    confidence: float = random()

    sleep(1)

    print(confidence, end='')

if __name__ == "__main__":
    main()