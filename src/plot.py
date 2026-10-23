# coding: utf-8

import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path


def main():
    fig, ax = plt.subplots(figsize=(8, 6))

    cwd = Path.cwd()
    result_path = cwd / "src/results/"

    ys = []
    for res in result_path.glob("*.csv"):
        print(res.stem)
        x, y = np.loadtxt(res, delimiter=",", skiprows=1, unpack=True)

        ax.plot(x ,y, label=res.stem.split("_")[-1])
        ax.set_xlabel("x")
        ax.set_ylabel("u")
        ax.grid()
        ax.legend()

    plt.show()



if __name__ == "__main__":
    main()