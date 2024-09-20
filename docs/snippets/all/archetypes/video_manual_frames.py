"""Manual use of individual video frame references."""
# TODO(#7298): ⚠️ Video is currently only supported in the Rerun web viewer.
# TODO(#7420): This sample doesn't render yet.

import sys

import rerun as rr
import rerun.blueprint as rrb

if len(sys.argv) < 2:
    # TODO(#7354): Only mp4 is supported for now.
    print(f"Usage: {sys.argv[0]} <path_to_video.[mp4]>")
    sys.exit(1)

rr.init("rerun_example_asset_video_manual_frames", spawn=True)

# Log video asset which is referred to by frame references.
rr.set_time_seconds("video_time", 0.0)
video_asset = rr.AssetVideo(path=sys.argv[1])
rr.log("video_asset", video_asset, static=True)


rr.log(
    "delay0",
    rr.VideoFrameReference(
        timestamp=rr.components.VideoTimestamp(seconds=0.0),
        video_reference="video_asset",
    ),
)
rr.log(
    "delay1",
    rr.VideoFrameReference(
        timestamp=rr.components.VideoTimestamp(seconds=1.0),
        video_reference="video_asset",
    ),
)
rr.log(
    "delay2",
    rr.VideoFrameReference(
        timestamp=rr.components.VideoTimestamp(seconds=2.0),
        video_reference="video_asset",
    ),
)
rr.log(
    "delay3",
    rr.VideoFrameReference(
        timestamp=rr.components.VideoTimestamp(seconds=3.0),
        video_reference="video_asset",
    ),
)

# Send automatically determined video frame timestamps.
frame_timestamps_ns = video_asset.read_frame_timestamps_ns()
rr.send_columns(
    "delay0",
    times=[rr.TimeNanosColumn("video_time", frame_timestamps_ns)],
    components=[rr.VideoFrameReference.indicator(), rr.components.VideoTimestamp.nanoseconds(frame_timestamps_ns)],
)
rr.send_columns(
    "delay1",
    times=[rr.TimeNanosColumn("video_time", frame_timestamps_ns)],
    components=[
        rr.VideoFrameReference.indicator(),
        rr.components.VideoTimestamp.nanoseconds(frame_timestamps_ns - 1000000000.0),
    ],
)
rr.send_columns(
    "delay2",
    times=[rr.TimeNanosColumn("video_time", frame_timestamps_ns)],
    components=[
        rr.VideoFrameReference.indicator(),
        rr.components.VideoTimestamp.nanoseconds(frame_timestamps_ns - 2000000000.0),
    ],
)
rr.send_columns(
    "delay3",
    times=[rr.TimeNanosColumn("video_time", frame_timestamps_ns)],
    components=[
        rr.VideoFrameReference.indicator(),
        rr.components.VideoTimestamp.nanoseconds(frame_timestamps_ns - 3000000000.0),
    ],
)


# Send blueprint that shows two 2D views next to each other.
rr.send_blueprint(
    rrb.Grid(
        rrb.Spatial2DView(origin="delay0"),
        rrb.Spatial2DView(origin="delay1"),
        rrb.Spatial2DView(origin="delay2"),
        rrb.Spatial2DView(origin="delay3"),
    )
)
