# Default: Show help menu
default:
    @just help

# ============================================================================
# Development Commands
# ============================================================================
import 'justfiles/development/web.just'
import 'justfiles/development/run.just'
import 'justfiles/development/dev.just'
import 'justfiles/development/example.just'
import 'justfiles/development/mdviewer.just'

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
import 'justfiles/utilities/screenshot.just'
import 'justfiles/utilities/package.just'
import 'justfiles/utilities/clean.just'
import 'justfiles/utilities/pub.just'
import 'justfiles/utilities/demo-record.just'
import 'justfiles/utilities/demo-interactive.just'
import 'justfiles/utilities/demo-replay.just'
import 'justfiles/utilities/demo-upload.just'
import 'justfiles/utilities/demo-gif.just'
import 'justfiles/utilities/demo-md.just'
import 'justfiles/utilities/demo-term.just'
import 'justfiles/utilities/demo-codediff.just'
import 'justfiles/utilities/demo-aichat.just'
import 'justfiles/utilities/demo-split.just'
import 'justfiles/utilities/demo-clean.just'
