window.BENCHMARK_DATA = {
  "lastUpdate": 1792589089609,
  "repoUrl": "https://github.com/moq-dev/moq",
  "entries": {
    "MoQ nightly throughput": [
      {
        "commit": {
          "author": {
            "name": "Luke Curley",
            "username": "kixelated",
            "email": "kixelated@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f13283b302ea86fb2f76af59f75489ab9a262c80",
          "message": "feat(demo): add a CARLA teleop demo (#5621)\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>\nCo-authored-by: Luke Curley <432854+kixelated@users.noreply.github.com>\nCo-authored-by: GPT-5 <noreply@openai.com>",
          "timestamp": "2026-10-20T08:39:45Z",
          "url": "https://github.com/moq-dev/moq/commit/f13283b302ea86fb2f76af59f75489ab9a262c80"
        },
        "date": 1792501729806,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "bench-ffi/rust-net/throughput/size=100/pub=1/sub=1",
            "value": 330.61717879371423,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=1",
            "value": 1864.3908258919935,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=65536/pub=1/sub=1",
            "value": 13386.52917542947,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1048576/pub=1/sub=1",
            "value": 13536.15343744316,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=4",
            "value": 4773.530415276492,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=1",
            "value": 1781.7785492381206,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=4",
            "value": 4550.6631106118075,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=100/pub=1/sub=1",
            "value": 68.57644331298096,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=1",
            "value": 576.9053249794918,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=65536/pub=1/sub=1",
            "value": 6949.068050052492,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1048576/pub=1/sub=1",
            "value": 7498.932208324513,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=4",
            "value": 742.4256301338369,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=1",
            "value": 546.3557964426078,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=4",
            "value": 706.7389730720178,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=100/pub=1/sub=1",
            "value": 22.6973,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=1",
            "value": 65.0804,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=65536/pub=1/sub=1",
            "value": 82.4462,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1048576/pub=1/sub=1",
            "value": 76.6543,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=4",
            "value": 67.3545,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=1",
            "value": 66.4884,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=4",
            "value": 67.6273,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=100/pub=1/sub=1",
            "value": 7.878258191352555,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=1",
            "value": 66.91970569545633,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=65536/pub=1/sub=1",
            "value": 324.824124434807,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1048576/pub=1/sub=1",
            "value": 249.9026737171371,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=4",
            "value": 93.92616427500664,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=1",
            "value": 75.5348243626966,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=4",
            "value": 98.23303577568466,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=100/pub=1/sub=1",
            "value": 8.510143280415337,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=1",
            "value": 75.61966349493792,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=4",
            "value": 179.9551313807728,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=1",
            "value": 139.92725347415907,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=4",
            "value": 255.09232390058236,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1048576/pub=1/sub=1",
            "value": 359.6937203472361,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=65536/pub=1/sub=1",
            "value": 375.06165422718584,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=100/pub=1/sub=1",
            "value": 1.2201502247878746,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=1",
            "value": 15.26567024796719,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=65536/pub=1/sub=1",
            "value": 206.7443329341433,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1048576/pub=1/sub=1",
            "value": 333.2107246728338,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=4",
            "value": 26.270390812226157,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=1",
            "value": 14.19046995386741,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=4",
            "value": 20.89563526470326,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=100/pub=1/sub=1",
            "value": 1.1255339297791611,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=1",
            "value": 10.293224316676707,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=65536/pub=1/sub=1",
            "value": 215.9159136936877,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1048576/pub=1/sub=1",
            "value": 342.8043642694111,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=4",
            "value": 17.54562608035285,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=1",
            "value": 12.209100429732274,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=4",
            "value": 19.94464761894559,
            "unit": "MB/s"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Luke Curley",
            "username": "kixelated",
            "email": "kixelated@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f13283b302ea86fb2f76af59f75489ab9a262c80",
          "message": "feat(demo): add a CARLA teleop demo (#5621)\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>\nCo-authored-by: Luke Curley <432854+kixelated@users.noreply.github.com>\nCo-authored-by: GPT-5 <noreply@openai.com>",
          "timestamp": "2026-10-20T08:39:45Z",
          "url": "https://github.com/moq-dev/moq/commit/f13283b302ea86fb2f76af59f75489ab9a262c80"
        },
        "date": 1792520372361,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "bench-ffi/rust-net/throughput/size=100/pub=1/sub=1",
            "value": 330.61717879371423,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=1",
            "value": 1864.3908258919935,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=65536/pub=1/sub=1",
            "value": 13386.52917542947,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1048576/pub=1/sub=1",
            "value": 13536.15343744316,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=4",
            "value": 4773.530415276492,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=1",
            "value": 1781.7785492381206,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=4",
            "value": 4550.6631106118075,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=100/pub=1/sub=1",
            "value": 68.57644331298096,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=1",
            "value": 576.9053249794918,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=65536/pub=1/sub=1",
            "value": 6949.068050052492,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1048576/pub=1/sub=1",
            "value": 7498.932208324513,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=4",
            "value": 742.4256301338369,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=1",
            "value": 546.3557964426078,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=4",
            "value": 706.7389730720178,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=100/pub=1/sub=1",
            "value": 22.6973,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=1",
            "value": 65.0804,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=65536/pub=1/sub=1",
            "value": 82.4462,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1048576/pub=1/sub=1",
            "value": 76.6543,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=4",
            "value": 67.3545,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=1",
            "value": 66.4884,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=4",
            "value": 67.6273,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=100/pub=1/sub=1",
            "value": 7.878258191352555,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=1",
            "value": 66.91970569545633,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=65536/pub=1/sub=1",
            "value": 324.824124434807,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1048576/pub=1/sub=1",
            "value": 249.9026737171371,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=4",
            "value": 93.92616427500664,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=1",
            "value": 75.5348243626966,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=4",
            "value": 98.23303577568466,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=100/pub=1/sub=1",
            "value": 8.510143280415337,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=1",
            "value": 75.61966349493792,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=4",
            "value": 179.9551313807728,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=1",
            "value": 139.92725347415907,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=4",
            "value": 255.09232390058236,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1048576/pub=1/sub=1",
            "value": 359.6937203472361,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=65536/pub=1/sub=1",
            "value": 375.06165422718584,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=100/pub=1/sub=1",
            "value": 1.2201502247878746,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=1",
            "value": 15.26567024796719,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=65536/pub=1/sub=1",
            "value": 206.7443329341433,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1048576/pub=1/sub=1",
            "value": 333.2107246728338,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=4",
            "value": 26.270390812226157,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=1",
            "value": 14.19046995386741,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=4",
            "value": 20.89563526470326,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=100/pub=1/sub=1",
            "value": 1.1255339297791611,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=1",
            "value": 10.293224316676707,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=65536/pub=1/sub=1",
            "value": 215.9159136936877,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1048576/pub=1/sub=1",
            "value": 342.8043642694111,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=4",
            "value": 17.54562608035285,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=1",
            "value": 12.209100429732274,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=4",
            "value": 19.94464761894559,
            "unit": "MB/s"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Luke Curley",
            "username": "kixelated",
            "email": "kixelated@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f13283b302ea86fb2f76af59f75489ab9a262c80",
          "message": "feat(demo): add a CARLA teleop demo (#5621)\n\nCo-authored-by: Claude Opus 5.5 <noreply@anthropic.com>\nCo-authored-by: Luke Curley <432854+kixelated@users.noreply.github.com>\nCo-authored-by: GPT-5 <noreply@openai.com>",
          "timestamp": "2026-10-20T08:39:45Z",
          "url": "https://github.com/moq-dev/moq/commit/f13283b302ea86fb2f76af59f75489ab9a262c80"
        },
        "date": 1792520489882,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "bench-ffi/rust-net/throughput/size=100/pub=1/sub=1",
            "value": 330.61717879371423,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=1",
            "value": 1864.3908258919935,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=65536/pub=1/sub=1",
            "value": 13386.52917542947,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1048576/pub=1/sub=1",
            "value": 13536.15343744316,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=4",
            "value": 4773.530415276492,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=1",
            "value": 1781.7785492381206,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=4",
            "value": 4550.6631106118075,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=100/pub=1/sub=1",
            "value": 68.57644331298096,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=1",
            "value": 576.9053249794918,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=65536/pub=1/sub=1",
            "value": 6949.068050052492,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1048576/pub=1/sub=1",
            "value": 7498.932208324513,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=4",
            "value": 742.4256301338369,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=1",
            "value": 546.3557964426078,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=4",
            "value": 706.7389730720178,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=100/pub=1/sub=1",
            "value": 22.6973,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=1",
            "value": 65.0804,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=65536/pub=1/sub=1",
            "value": 82.4462,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1048576/pub=1/sub=1",
            "value": 76.6543,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=4",
            "value": 67.3545,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=1",
            "value": 66.4884,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=4",
            "value": 67.6273,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=100/pub=1/sub=1",
            "value": 7.878258191352555,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=1",
            "value": 66.91970569545633,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=65536/pub=1/sub=1",
            "value": 324.824124434807,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1048576/pub=1/sub=1",
            "value": 249.9026737171371,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=4",
            "value": 93.92616427500664,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=1",
            "value": 75.5348243626966,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=4",
            "value": 98.23303577568466,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=100/pub=1/sub=1",
            "value": 8.510143280415337,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=1",
            "value": 75.61966349493792,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=4",
            "value": 179.9551313807728,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=1",
            "value": 139.92725347415907,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=4",
            "value": 255.09232390058236,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1048576/pub=1/sub=1",
            "value": 359.6937203472361,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=65536/pub=1/sub=1",
            "value": 375.06165422718584,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=100/pub=1/sub=1",
            "value": 1.2201502247878746,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=1",
            "value": 15.26567024796719,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=65536/pub=1/sub=1",
            "value": 206.7443329341433,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1048576/pub=1/sub=1",
            "value": 333.2107246728338,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=4",
            "value": 26.270390812226157,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=1",
            "value": 14.19046995386741,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=4",
            "value": 20.89563526470326,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=100/pub=1/sub=1",
            "value": 1.1255339297791611,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=1",
            "value": 10.293224316676707,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=65536/pub=1/sub=1",
            "value": 215.9159136936877,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1048576/pub=1/sub=1",
            "value": 342.8043642694111,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=4",
            "value": 17.54562608035285,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=1",
            "value": 12.209100429732274,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=4",
            "value": 19.94464761894559,
            "unit": "MB/s"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Melanie Riise",
            "username": "mriise",
            "email": "me@mriise.net"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "bc64b9c52561209322dcb8dec9992bf867f499f7",
          "message": "fix(net): refuse a revoked subscription while its SUBSCRIBE is still pending (#6061)\n\nCo-authored-by: Claude Fable 5.1 <noreply@anthropic.com>\nCo-authored-by: Luke Curley <kixelated@gmail.com>",
          "timestamp": "2026-10-21T05:04:56Z",
          "url": "https://github.com/moq-dev/moq/commit/bc64b9c52561209322dcb8dec9992bf867f499f7"
        },
        "date": 1792589089380,
        "tool": "customBiggerIsBetter",
        "benches": [
          {
            "name": "bench-ffi/rust-net/throughput/size=100/pub=1/sub=1",
            "value": 309.3294828850396,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=1",
            "value": 1693.2144899606355,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=65536/pub=1/sub=1",
            "value": 13121.348576875549,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1048576/pub=1/sub=1",
            "value": 12930.840076190672,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=1/sub=4",
            "value": 4591.929660828687,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=1",
            "value": 1440.1863917549833,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-net/throughput/size=1024/pub=4/sub=4",
            "value": 4089.450256411346,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=100/pub=1/sub=1",
            "value": 69.00923986155097,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=1",
            "value": 573.8050708612742,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=65536/pub=1/sub=1",
            "value": 6770.954200328875,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1048576/pub=1/sub=1",
            "value": 6653.394355687621,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=1/sub=4",
            "value": 739.584759589979,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=1",
            "value": 527.5857041911794,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/rust-ffi/throughput/size=1024/pub=4/sub=4",
            "value": 703.7809192604783,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=100/pub=1/sub=1",
            "value": 22.8466,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=1",
            "value": 65.0058,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=65536/pub=1/sub=1",
            "value": 82.0986,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1048576/pub=1/sub=1",
            "value": 76.1506,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=1/sub=4",
            "value": 67.287,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=1",
            "value": 66.1839,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/cpp/throughput/size=1024/pub=4/sub=4",
            "value": 67.4222,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=100/pub=1/sub=1",
            "value": 9.470939254803463,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=1",
            "value": 75.91909956031502,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=65536/pub=1/sub=1",
            "value": 263.2174991458219,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1048576/pub=1/sub=1",
            "value": 208.79732695072462,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=1/sub=4",
            "value": 110.7332142985536,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=1",
            "value": 84.42893452389855,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/dart/throughput/size=1024/pub=4/sub=4",
            "value": 114.15437411983261,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=100/pub=1/sub=1",
            "value": 8.958541219437958,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=1",
            "value": 78.15264837396677,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=1/sub=4",
            "value": 203.090572208818,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=1",
            "value": 136.82860866010185,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1024/pub=4/sub=4",
            "value": 220.5088931812131,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=1048576/pub=1/sub=1",
            "value": 368.8718363093468,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/go/throughput/size=65536/pub=1/sub=1",
            "value": 369.95216879236466,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=100/pub=1/sub=1",
            "value": 1.0299207233016494,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=1",
            "value": 13.623504694366623,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=65536/pub=1/sub=1",
            "value": 211.07132540226806,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1048576/pub=1/sub=1",
            "value": 449.79315777574067,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=1/sub=4",
            "value": 26.114897045392546,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=1",
            "value": 14.875761099080242,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/kotlin/throughput/size=1024/pub=4/sub=4",
            "value": 19.259224050419736,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=100/pub=1/sub=1",
            "value": 1.2873199202936298,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=1",
            "value": 11.611788465030175,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=65536/pub=1/sub=1",
            "value": 224.60740709567045,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1048576/pub=1/sub=1",
            "value": 345.7455099905332,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=1/sub=4",
            "value": 19.66044002639729,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=1",
            "value": 13.421516137663613,
            "unit": "MB/s"
          },
          {
            "name": "bench-ffi/python/throughput/size=1024/pub=4/sub=4",
            "value": 22.407625383024705,
            "unit": "MB/s"
          }
        ]
      }
    ]
  }
}