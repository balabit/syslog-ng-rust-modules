class ParserForImport:
    def init(self, options):
        return True
    def parse(self, logmsg, message):
        print("foo")
        print("bar")
        print(message)
        logmsg["foo"] = "bar"
        print(logmsg["foo"])
        return True
