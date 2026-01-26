import sys

try:
    import pulsectl
except ImportError:
    print("Error: pulsectl is not installed, please install it!", file=sys.stderr)
    sys.exit(1)

pulse = pulsectl.Pulse("audiocontrol")

def set_volume(target, volume):
    volume = max(0.0, min(1.0, volume))

    if target == "system":
        sink = pulse.get_sink_by_name(pulse.server_info().default_sink_name)
        pulse.volume_set_all_chans(sink, volume)
        return

    # per-app volume
    for sink_input in pulse.sink_input_list():
        app = sink_input.proplist.get("application.name", "").lower()
        if target.lower() in app:
            pulse.volume_set_all_chans(sink_input, volume)

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue

    parts = line.split()
    if len(parts) != 3 or parts[0] != "SET":
        continue

    _, target, value = parts
    try:
        set_volume(target, float(value))
    except Exception as e:
        print(f"ERROR {e}", file=sys.stderr)
