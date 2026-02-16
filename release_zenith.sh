#!/bin/bash
set -e

echo "🚀 Packaging Zenith OS Suite..."

# 1. Create Dist Directory
mkdir -p dist/Zenith_Product
mkdir -p dist/Zenith_Research_Kernel

# 2. Build Daemon (The Product)
echo "📦 Building Zenith Daemon (App)..."
cargo build --release -p zenith_daemon
cp target/release/zenith_daemon dist/Zenith_Product/
cp -r zenith_daemon/static dist/Zenith_Product/
echo "✅ Daemon packaged."

# 3. Package Kernel (The Research)
echo "🧬 Packaging Zenith Kernel (Research)..."
# We assume the kernel is built. If not, we'd run make. 
# copying the run script and kernel binary if they exist
if [ -f "build/aarch64/kernel.img" ]; then
    cp build/aarch64/kernel.img dist/Zenith_Research_Kernel/
    cp Makefile dist/Zenith_Research_Kernel/
    echo "✅ Kernel packaged."
else
    echo "⚠️ Kernel image not found. Skipping kernel package (Run 'make' first)."
fi

# 4. Add Documentation
echo "bdd Copying Documentation..."
# Copy the artifacts 
cp /Users/rajkrish0608/.gemini/antigravity/brain/7731931b-3f8a-46af-b54e-962f6c5c6e1a/user_guide.md dist/README_FIRST.md
cp /Users/rajkrish0608/.gemini/antigravity/brain/7731931b-3f8a-46af-b54e-962f6c5c6e1a/zenith_features_simple.md dist/Zenith_Product/FEATURES.md

echo "✨ Done! Release is ready in 'dist/' folder."
echo "   - Zenith_Product:  Send this to users."
echo "   - Zenith_Research: For developers."
