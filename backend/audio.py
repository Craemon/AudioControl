import sys

try:
    import pulsectl
except ImportError:
    print("Error: pulsectl is not installed, please install it!", file=sys.stderr)
    sys.exit(1)

pulse = pulsectl.Pulse("audiocontrol")

def match_sink_input(target: str, sink_input) -> bool:
    target_lower = target.lower()
    props = sink_input.proplist

    # Collect all relevant properties that might identify the application
    candidates = [
        props.get("application.name", ""),
        props.get("application.process.binary", ""),
        props.get("media.name", ""),
        props.get("node.name", ""),
    ]

    # Return True if target matches any property
    return any(target_lower in val.lower() for val in candidates if val)

def set_volume(target: str, volume: float):
    volume = max(0.0, min(1.0, volume))

    if target.lower() == "system":
        sink = pulse.get_sink_by_name(pulse.server_info().default_sink_name)
        pulse.volume_set_all_chans(sink, volume)
        return

    # per-app volume
    for sink_input in pulse.sink_input_list():
        if match_sink_input(target, sink_input):
            pulse.volume_set_all_chans(sink_input, volume)

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue

    parts = line.rsplit(maxsplit=1)
    if len(parts) != 2:
        continue

    cmd_and_target, value_str = parts
    cmd_parts = cmd_and_target.split(maxsplit=1)
    if len(cmd_parts) != 2 or cmd_parts[0] != "SET":
        continue

    target = cmd_parts[1]

    try:
        set_volume(target, float(value_str))
    except Exception as e:
        print(f"ERROR {e}", file=sys.stderr)