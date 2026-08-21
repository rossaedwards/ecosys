import torch
import torch.nn as nn
from torchvision.models import resnet18

def slice_model(model: nn.Module, split_layer: str):
    layers = list(model.children())
    split_idx = None
    for i, layer in enumerate(layers):
        if layer.__class__.__name__ == split_layer:
            split_idx = i + 1
            break
    if split_idx is None:
        raise ValueError(f"Layer {split_layer} not found in model")

    shard1 = nn.Sequential(*layers[:split_idx])
    shard2 = nn.Sequential(*layers[split_idx:])
    return shard1, shard2

if __name__ == "__main__":
    model = resnet18()
    s1, s2 = slice_model(model, "ReLU")
    print("Shard 1:", s1)
    print("Shard 2:", s2)
# To run: python slice_model.py
# Ensure you have torch and torchvision installed