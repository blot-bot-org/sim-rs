'''
    Program to automatically draw the instruction file as it is updated.
    This can be paired with `feh` to have the image window update in realtime, as the `ins.json` file is updated.
    Ensure you have ran `cargo build --release` first, so this script has a binary to run.
'''

import os
import subprocess
import time


def generate_image():
    subprocess.call(["./target/release/sim-rs"])


last_time = None
while True:
    time.sleep(1)

    new_time = os.stat("./ins.json").st_mtime
    if not new_time == last_time:
        generate_image()
        last_time = new_time
        
