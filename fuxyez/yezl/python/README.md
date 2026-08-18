# Python Yezl Bridge

Python integration for Fuxyez enabling \.fuxpy\ files.

## Features

- Import any Python package
- Call Python functions from Fuxyez
- Type conversion between Fuxyez and Python types
- Async/await support

## Usage

\\\uxyez
// data_analysis.fuxpy
import pandas as pd
import numpy as np

sigil analyze_csv {
    let df = pd.read_csv("data.csv")
    let summary = df.describe()
    echo summary
}
\\\

## Type Mapping

| Fuxyez Type | Python Type |
|-------------|-------------|
| Spinon | dict |
| Lattice | list |
| Oracle | callable |
| String | str |
| Integer | int |
| Float | float |
