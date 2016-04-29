def info(msg): pass
def trace(msg): pass
def warning(msg): pass
def error(msg): pass
def debug(msg): pass

class LoggingCallbacksAreNotOverriden:
    def init(self, options):
        info("INFO")
        warning("WARNING")
        trace("TRACE")
        error("ERROR")
        debug("DEBUG")
