
echo "🤐 Packaging project template..."
mkdir -p target
cd template
# -r for recursive, -X to ignore extra file attributes (cleaner)
zip -r ../target/template.zip . -x "*.DS_Store"
cd ..

echo "🔄 Building the project by embedding template"
cargo build