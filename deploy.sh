export SHUTTLE_BETA=true
SECONDS=0
cargo shuttle deploy --allow-dirty
echo time elapsed: $SECONDS seconds.

