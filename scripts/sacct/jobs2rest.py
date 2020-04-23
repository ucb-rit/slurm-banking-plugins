#!/usr/bin/env python
# This script reads job input data from standard input and loads it into postgres
# The input format is delimited by | and fields listed below in the code

import requests
import fileinput
import datetime
import pwd

import rest_config

line_count = 0
for line in fileinput.input():
  # job_id|account|type|department|cpus|partition|state|req_nodes|alloc_nodes|raw_time|cpu_time|end_time|start_time|node_list|user
  print(line)
  parts = line.split('|')
  job_id = parts[0]
  account = parts[1]
  account_type = parts[2]
  department = parts[3]
  cpus = parts[4]
  partition = parts[5]
  state = parts[6]
  req_nodes = parts[7]
  alloc_nodes = parts[8]
  raw_time = float(parts[9])
  cpu_time = float(parts[10])
  end_time = str(datetime.datetime.utcfromtimestamp(int(parts[11])))
  start_time = str(datetime.datetime.utcfromtimestamp(int(parts[12])))
  nodes = list(filter(lambda x: x, parts[13].split(',')))
  user = parts[14]

  data = {
    'userid': str(pwd.getpwnam(user).pw_uid),
    'jobslurmid': job_id, 
    'startdate': start_time, 
    'end_date': end_time, 
    'accountid': account,
    # 'type': account_type,
    # 'department': department,
    'num_cpus': int(cpus),
    'jobstatus': state,
    'partition': partition,
    'num_req_nodes': int(req_nodes),
    'num_alloc_nodes': int(alloc_nodes),
    'raw_time': raw_time,
    'cpu_time': cpu_time,
    'nodes': [{'name': host} for host in nodes],
    'amount': 0
  }
  headers = {'Authorization': rest_config.TOKEN}
  r = requests.put(rest_config.MYBRC_REST_URL + '/jobs/' + job_id + '/', data=data, headers=headers)
  print(data)
  print(r.content)

  # if nodes and nodes[0]:
  #     for node in nodes:
  #         job_node_data = { 'job': job_id, 'hostname': node }
  #         requests.put('http://scgup-dev.lbl.gov:8888/mybrc-rest/jobs/' + job_id + '/', data=job_node_data)
  line_count += 1

print('Sent {} rows'.format(line_count))
