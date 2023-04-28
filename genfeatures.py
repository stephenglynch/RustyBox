import kconfiglib
import re

pattern = re.compile('^UTIL_([A-Z]+)$')


def sym_util_to_feature(sym):
    m = pattern.match(sym.name)
    if m:
        return f'{m.group(1).lower()}-util'


if __name__ == '__main__':
    kconf = kconfiglib.Kconfig()
    kconf.load_config()

    features = []

    for sym in kconf.unique_defined_syms:
        if sym.str_value == "y":
            features.append(sym_util_to_feature(sym))
    
    print(' '.join(features))
