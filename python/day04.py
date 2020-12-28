import re


def read_doc(line):
    return dict([x.split(':') for x in line.split()])


REQUIRED_FIELDS = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']


def is_valid1(passport):
    return all(f in passport for f in REQUIRED_FIELDS)


def in_range(int_str, minv, maxv):
    try:
        return minv <= int(int_str) <= maxv
    except:
        return False


def is_valid_height(height_str):
    match = re.match(r"^(?P<in>\d+)in|(?P<cm>\d+)cm$", height_str)
    if match == None:
        return False
    hcm, hin = match.group('cm'), match.group('in')
    if hcm != None:
        return in_range(hcm, 150, 193)
    elif hin != None:
        return in_range(hin, 59, 76)
    else:
        return False


VALID_EYE_COLORS = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']


def is_valid2(passport):
    return is_valid1(passport) \
        and in_range(passport['byr'], 1920, 2002) \
        and in_range(passport['iyr'], 2010, 2020) \
        and in_range(passport['eyr'], 2020, 2030) \
        and passport['ecl'] in VALID_EYE_COLORS \
        and re.match(r"^(\d{9})$", passport['pid']) is not None \
        and re.match(r"^#[0-9a-f]{6}$", passport['hcl']) is not None \
        and is_valid_height(passport['hgt'])


def solution():
    lines = open('../data/04.txt', 'r').read()
    passports = [read_doc(line) for line in lines.split("\n\n")]

    print(f'Answer 1: {sum(map(is_valid1, passports))}')
    print(f'Answer 2: {sum(map(is_valid2, passports))}')
