# Default: Show help menu
default:
    @just help

# ============================================================================
# Help Command
# ============================================================================

help:
    @echo ""
    @echo "\033[1;36m======================================\033[0m"
    @echo "\033[1;36m       Project Commands               \033[0m"
    @echo "\033[1;36m======================================\033[0m"
    @echo ""
    @echo "\033[1;35m  Most Common Commands:\033[0m"
    @echo "  just \033[0;33mrun\033[0m                      \033[0;32mRun compiled showcase demo\033[0m"
    @echo "  just \033[0;33mdev\033[0m                     \033[0;32mStart showcase demo\033[0m"
    @echo "  just \033[0;33mexample\033[0m                 \033[0;32mRun specific example\033[0m"
    @echo "  just \033[0;33mtest\033[0m                    \033[0;32mRun all tests\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                   \033[0;32mRun all checks\033[0m"
    @echo ""
    @echo "\033[1;35m  Development:\033[0m"
    @echo "  just \033[0;33mrun\033[0m                      \033[0;32mRun compiled showcase demo\033[0m"
    @echo "  just \033[0;33mdev\033[0m                     \033[0;32mStart showcase demo\033[0m"
    @echo "  just \033[0;33mweb-serve\033[0m               \033[0;32mServe web demo (localhost:8080)\033[0m"
    @echo "  just \033[0;33mweb-build\033[0m               \033[0;32mBuild web demo for production\033[0m"
    @echo "  just \033[0;33mexample <name>\033[0m          \033[0;32mRun specific example\033[0m"
    @echo ""
    @echo "\033[1;35m  Building:\033[0m"
    @echo "  just \033[0;33mbuild\033[0m                   \033[0;32mBuild with all features\033[0m"
    @echo ""
    @echo "\033[1;35m  Verification:\033[0m"
    @echo "  just \033[0;33mlint\033[0m                    \033[0;32mRun clippy linter\033[0m"
    @echo "  just \033[0;33mfmt-check\033[0m              \033[0;32mCheck formatting\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                  \033[0;32mRun all checks\033[0m"
    @echo ""
    @echo "\033[1;35m  Testing:\033[0m"
    @echo "  just \033[0;33mtest\033[0m                    \033[0;32mRun all tests\033[0m"
    @echo ""
    @echo "\033[1;35m  Utilities:\033[0m"
    @echo "  just \033[0;33mfmt\033[0m                     \033[0;32mFormat code\033[0m"
    @echo "  just \033[0;33mdoc\033[0m                     \033[0;32mBuild documentation\033[0m"
    @echo "  just \033[0;33mpackage\033[0m                 \033[0;32mPackage for crates.io (dry run)\033[0m"
    @echo "  just \033[0;33mclean\033[0m                   \033[0;32mClean build artifacts\033[0m"
    @echo "  just \033[0;33mpub\033[0m                     \033[0;32mPublish to crates.io\033[0m"
    @echo ""
    @echo "\033[1;35m  Demo Utilities:\033[0m"
    @echo "  just \033[0;33mdemo-record\033[0m             \033[0;32mRecord automated demo\033[0m"
    @echo "  just \033[0;33mdemo-interactive\033[0m       \033[0;32mRecord interactive demo\033[0m"
    @echo "  just \033[0;33mdemo-replay\033[0m            \033[0;32mReplay interactive demo\033[0m"
    @echo "  just \033[0;33mdemo-upload\033[0m            \033[0;32mUpload demo to asciinema.org\033[0m"
    @echo "  just \033[0;33mdemo-gif\033[0m                \033[0;32mConvert demo to GIF\033[0m"
    @echo "  just \033[0;33mdemo-mdz\033[0m                \033[0;32mRun standalone Markdown Widget demo\033[0m"
    @echo "  just \033[0;33mdemo-term\033[0m               \033[0;32mRun standalone Terminal Pane demo\033[0m"
    @echo "  just \033[0;33mdemo-clean\033[0m              \033[0;32mRemove all demo files\033[0m"
    @echo ""

# ============================================================================
# Development Commands
# ============================================================================
import 'justfiles/development/web.just'
import 'justfiles/development/run.just'
import 'justfiles/development/dev.just'
import 'justfiles/development/example.just'

# ============================================================================
# Building Commands
# ============================================================================
import 'justfiles/building/build.just'

# ============================================================================
# Verification Commands
# ============================================================================
import 'justfiles/verification/lint.just'
import 'justfiles/verification/fmt-check.just'
import 'justfiles/verification/check.just'

# ============================================================================
# Testing Commands
# ============================================================================
import 'justfiles/testing/test.just'

# ============================================================================
# Utilities Commands
# ============================================================================
import 'justfiles/utilities/fmt.just'
import 'justfiles/utilities/doc.just'
import 'justfiles/utilities/package.just'
import 'justfiles/utilities/clean.just'
import 'justfiles/utilities/pub.just'
import 'justfiles/utilities/demo-record.just'
import 'justfiles/utilities/demo-interactive.just'
import 'justfiles/utilities/demo-replay.just'
import 'justfiles/utilities/demo-upload.just'
import 'justfiles/utilities/demo-gif.just'
import 'justfiles/utilities/demo-mdz.just'
import 'justfiles/utilities/demo-term.just'
import 'justfiles/utilities/demo-clean.just'
