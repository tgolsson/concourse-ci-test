'''

'''


import time
import dataclasses
from typing import List

import requests


@dataclasses.dataclass
class Status:
    app: str
    message: str
    tags: List[str]
    epoch_seconds: int


@dataclasses.dataclass
class StatusList:
    events: List[Status]
    min_id: int


def post(endpoint: str,
         message: str,
         app: str,
         tags: List[str],):
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
    '''Retrieve a number of statuses from the Pathik backend

    :param endpoint: Pathik server to query, including protocol
    :param count: number of events to retrieve
    :min_id: minimum id to retrieve'''
    queries = {
        'last_id': min_id,
        'limit': count,
    }

    r = requests.get(
        f'{endpoint}/api/status',
        params=queries
    )

    if r.status_code == 200:
        body = r.json()
        return StatusList([Status(**k) for k in body['events']], body['last_id'])
