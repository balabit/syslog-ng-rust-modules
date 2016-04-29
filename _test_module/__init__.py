# Keep this class commented out
# class NonExistingParser: pass

class ExistingParser: pass

class CallableClass: pass

NotCallableObject = int()

class ClassWithInitMethod:
    def init(self, options):
        pass

class InitMethodReturnsNotNone:
    def init(self, options):
        return True

class ParserWithoutInitMethod: pass

class ParserClassWithGoodParseMethod:
    def parse(self, logmsg, input):
        return True

class ParserWithoutParseMethod: pass

class ParseMethodReturnsNotBoolean:
    def parse(self, logmsg, input):
        return None

class ParseReturnsTrue:
    def parse(self, logmsg, input):
        return True

class ParseReturnsFalse:
    def parse(self, logmsg, input):
        return False

class ExceptionIsRaisedInParseMethod:
    def parse(self, logmsg, input):
        raise TypeError("text")
        return False

class ExceptionIsRaisedInInitMethod:
    def init(self, options):
        raise TypeError("text")
        return True

class LoggingIsUsedInInitMethod:
    def init(self, options):
        info("INFO")
        warning("WARNING")
        trace("TRACE")
        error("ERROR")
