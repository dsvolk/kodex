import sys
from pathlib import Path

_EXAMPLES_ROOT = Path(__file__).resolve().parents[1]
if str(_EXAMPLES_ROOT) not in sys.path:
    sys.path.insert(0, str(_EXAMPLES_ROOT))

from _bootstrap import ensure_local_sdk_src, generated_sample_image_data_url, runtime_config

ensure_local_sdk_src()

from openai_kodex import ImageInput, Kodex, TextInput

IMAGE_DATA_URL = generated_sample_image_data_url()

with Kodex(config=runtime_config()) as kodex:
    thread = kodex.thread_start(model="gpt-5.4", config={"model_reasoning_effort": "high"})
    result = thread.turn(
        [
            TextInput("What is in this image? Give 3 bullets."),
            ImageInput(IMAGE_DATA_URL),
        ]
    ).run()

    print("Status:", result.status)
    print(result.final_response)
