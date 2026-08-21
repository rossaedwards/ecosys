from setuptools import setup, find_packages

setup(
    name="aurphyx-casino-sdk",
    version="0.1.0",
    description="Aurphyx Casino Python SDK",
    packages=find_packages(),
    install_requires=[
        "requests>=2.31.0",
    ],
    python_requires=">=3.10",
)

