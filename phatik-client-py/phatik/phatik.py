'''
Provides the API surface for Phatik client
'''


import time
import dataclasses
from typing import List

import requests

# pylint: disable=too-few-public-methods


@dataclasses.dataclass
class Status:
    '''A single status object'''
    app: str
    message: str
    tags: List[str]
    epoch_seconds: int


@dataclasses.dataclass
class StatusList:
    '''A set of statuses and the id of the last item'''
    events: List[Status]
    min_id: int


def post(endpoint: str,
         message: str,
         app: str,
         tags: List[str],) -> bool:
    '''Post a Status to the Phatik backend

    :param endpoint: Phatik server to query, including protocol
    :param message: primary message of the status
    :param app: app producing this status
    :param tags: list of tags for this status
    '''

    payload = {
        'message': message,
        'app': app,
        'tags': tags,
        'epoch_seconds': int(time.time())
    }

    return requests.post(
        f'{endpoint}/api/status',
        json=payload
    ).status_code == 201


def get(endpoint: str, count: int, min_id: id) -> StatusList:
    '''Retrieve a number of statuses from the Phatik backend

    :param endpoint: Phatik server to query, including protocol
    :param count: number of events to retrieve
    :param min_id: minimum id to retrieve'''
    queries = {
        'last_id': min_id,
        'limit': count,
    }

    res = requests.get(
        f'{endpoint}/api/status',
        params=queries
    )

    if res.status_code == 200:
        body = res.json()
        return StatusList([Status(**k) for k in body['events']],
                          body['last_id'])

    return None
