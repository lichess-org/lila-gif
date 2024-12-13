import os
import requests

colors = ["blue", "brown", "green", "ic", "pink", "purple"]
piece_dirs = [
    os.path.basename(f.path) for f in os.scandir(
        os.path.join(os.path.dirname(__file__), "piece")
    ) if f.is_dir()
]

exit_code = 0

for color in colors:
    for piece_set in piece_dirs:
        url = f"http://localhost:6175/image.gif?theme={color}&piece={piece_set}"

        response = requests.get(url)
        if response.status_code == 200:
            print(f"✅ {url}")
        else:
            print(f"❌ {url} - {response.text}")
            exit_code = 1

exit(exit_code)
