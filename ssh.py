#!/usr/bin/env python3

from pexpect import pxssh
import argparse
import pprint as pp

def main():
    parser = argparse.ArgumentParser(description='Remote Raspi Security Check, Python version')

    parser.add_argument('host', metavar='host', type=str, nargs='+', help='host IP address')

    args = parser.parse_args()

    commands = {
        'cpu': 'cat /proc/cpuinfo',
        'uname': 'uname -a',
        'eth': 'ifconfig wlan0 | grep ether',
        'groups': 'groups',
    }

    cc = []

    try:
        s = pxssh.pxssh()
        hostname = args.host
        username = 'pi'
        s.login(hostname, username)

        s.sendline(' HISTFILE=  unset HISTFILE')
        s.prompt()
        print(s.before.decode())

        for (cmd, line) in commands.items():
            s.sendline(line)
            s.prompt()
            cc.append({
                'name': cmd,
                'cmd': line,
                'out': s.before,
            })

        with open("./result_" + args.host , 'w') as out_file:
            pp.pprint(cc, stream=out_file)

        s.logout()

    except pxssh.ExceptionPxssh as e:
        print("pxssh failed on login.")
        print(e)


if __name__ == '__main__':
    main()
