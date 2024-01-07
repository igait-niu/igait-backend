from time import sleep
from random import random
import sys

def main():
    id: float = float(sys.argv[1])
    confidence: float = random() + id

    sleep(1)

    print(confidence, end='')

if __name__ == "__main__":
    main()