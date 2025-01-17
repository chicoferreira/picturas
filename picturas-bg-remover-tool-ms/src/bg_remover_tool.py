import random

from PIL import Image, ImageEnhance
import rembg
import numpy as np
import os

from .core.tool import Tool
from .bg_remover_request_message import BgRemoverParameters


class BgRemoverTool(Tool):
        

    def apply(self, parameters: BgRemoverParameters):
        """
        Remove the background of the input image and save the result.

        Args:
            parameters (BgRemoverParameters): bg-remover parameters.
        """
        input_image = Image.open(parameters.inputImageURI).convert("RGBA")

        input_array = np.array(input_image)

        output_array = rembg.remove(input_array)

        output_image = Image.fromarray(output_array)

        final_image = output_image.convert("RGB")
        
        output_dir = os.path.dirname(parameters.outputImageURI)
        if output_dir and not os.path.exists(output_dir):
            os.makedirs(output_dir)

        final_image.save(parameters.outputImageURI)
