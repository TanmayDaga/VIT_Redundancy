import timm
import torch
from timm.data import resolve_data_config
from timm.data.transforms_factory import create_transform
from torchvision.transforms import Compose
from PIL import Image


def pre_processor(path_of_image: str, model_name: str) -> torch.Tensor:

    img = Image.open(path_of_image).convert("RGB")

    config: dict = resolve_data_config({}, model="vit_base_patch16_224")
    transform: Compose = create_transform(**config)

    input_tensor: torch.Tensor = transform(img)
    input_tensor = input_tensor.unsqueeze(0)
    return input_tensor


def main():
    print(pre_processor("./img.jpeg", "vit_base_patch16_224").shape)

    model = timm.create_model("vit_base_patch16_224", pretrained=True)
    layer: torch.nn.modules.conv.Conv2d = model.patch_embed.proj
    print(layer.weight.shape)


if __name__ == "__main__":
    main()
