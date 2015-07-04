echo "Cleaning build..."
cargo clean
echo "Merging latest changes from master..."
git merge master
mkdir docs/
echo "Building API Documentation..."
sh build_api_docs.sh
echo "Building KISS-UI Users Guide..."
sh build_guide.sh
