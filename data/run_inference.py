from time import sleep
from random import random
import sys
from os.path import exists, dirname, realpath

def main():
    id: int = int(sys.argv[1])

    if not exists(f"{dirname(realpath(__file__))}/queue/{id}.mp4"):
        print(f"File {dirname(realpath(__file__))}/queue/{id}.mp4 does not exist!")
        return

    confidence: float = random() + id

    sleep(1)

    print(confidence, end='')

if __name__ == "__main__":
    main()