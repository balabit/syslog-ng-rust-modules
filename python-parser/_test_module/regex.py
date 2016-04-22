import re

class RegexParser:
    def init(self, options):
        pattern = options["regex"]
        self.regex = re.compile(pattern)

    def parse(self, logmsg, message):
        match = self.regex.match(message)
        if match is not None:
            for key, value in match.groupdict().items():
                logmsg[key] = value
            return True
        else:
            return False
