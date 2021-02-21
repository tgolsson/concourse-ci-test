'''
CLI implementation for Phatik library
'''

import argparse

from .phatik import post, get


def post_command(args: argparse.Namespace):
    '''Command handler for post'''
    post(args.endpoint,
         args.message,
         args.app,
         args.tags)


def list_command(args: argparse.Namespace):
    '''Command handler for get'''
    response = get(args.endpoint, args.count, args.min_id)
    if response is None:
        return

    if args.format == 'raw':
        print(response)

    elif args.format == 'json':
        import json
        from dataclasses import asdict
        print(json.dumps(asdict(response)))


class ArgparseHelper(argparse._HelpAction):
    # pylint: disable=protected-access,too-few-public-methods
    '''
    Used to help print top level '--help' arguments from argparse
    when used with subparsers
    Usage:
    from scutils.arparse_helper import ArgparseHelper
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument('-h', '--help', action=ArgparseHelper,
                        help='show this help message and exit')
    # add subparsers below these lines
    '''

    def __call__(self, parser, namespace, values, option_string=None):
        parser.print_help()
        print()

        subparsers_actions = [
            action for action in parser._actions
            if isinstance(action, argparse._SubParsersAction)]
        for subparsers_action in subparsers_actions:
            for choice, subparser in list(subparsers_action.choices.items()):
                print("Command '{}'".format(choice))
                print(subparser.format_usage())

        parser.exit()


def _parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description='Push and pull data from a Phatik server',
        add_help=False
    )

    parser.add_argument('-h', '--help',
                        action=ArgparseHelper,
                        help='show this help message and exit')

    base_parser = argparse.ArgumentParser(add_help=False)
    base_parser.add_argument("--endpoint", type=str, required=True,
                             help='the server url to connect to')
    subparser = parser.add_subparsers(help='commands', dest='command')
    subparser.required = True

    post_parser = subparser.add_parser(
        'post',
        help='post a status to Phatik',
        parents=[base_parser]
    )
    post_parser.add_argument('message', type=str, help='the message to post')
    post_parser.add_argument('app', type=str, help='the source application')
    post_parser.add_argument('tags', type=str, nargs='*',
                             help='the source application', default=[])
    post_parser.set_defaults(func=post_command)

    list_parser = subparser.add_parser(
        'list',
        help='list recent statuses from the backend',
        parents=[base_parser]
    )
    list_parser.add_argument('--min-id', type=int,
                             help='lowest id to fetch for incremental queries')
    list_parser.add_argument('--count', type=int,
                             help='number of messages to fetch')
    list_parser.add_argument('-f', '--format', type=str,
                             help='format of output', choices=['json', 'raw'],
                             default='json')
    list_parser.set_defaults(func=list_command)

    return parser.parse_args()


def main():
    '''Main entrypoint for CLI'''
    args = _parse_arguments()
    args.func(args)
