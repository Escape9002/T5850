# echo-client.py

import time
import re
import numpy as np

import socket
HOST = "localhost"  # The server's hostname or IP address
PORT = 4000  # The port used by the server

player_id = "404"
own_pos = np.full(shape =(2,1), fill_value=0)
other_pos = list()

board = np.full(shape=(2, 2),fill_value=-1)

def clamp_pos(value, min, max):
    if(value >= max):
        value = min
    elif (value <= min):
        value = max

    return value

#   [ ...
#       [msg, value, values],
#       [msg, value, value],
#    ... 
#   ]

def req_data(msg):
    msg = msg.decode('utf-8')
    msg = msg.split("\n")
    data = list()

    for package in msg:

        re.split('motd |die |pos |message |tick | lose |player', package)
        
        package = package.split("|")
        data.append(package)
        
    print(data)
    return data        

def get_closest_player(other_pos):
    dist = list()
    for player_id, pos in other_pos:
        dist.append((player_id, np.linalg.norm(own_pos - pos)))
    distance = np.array(dist)

    id_max = 0
    for i in range(len(distance)):
        if(distance[id_max][1]< distance[i][1]):
            id_max = i
                          
    #print(f'show distances: {distance[id_max]} \n and id: {id_max}')
    return id_max

#def where_to_go(closest_player_pos):

player_count = 0
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(b"join|T5850|GM965\n")

    print("connection suuuuucs")
    while(True):
        data = req_data(s.recv(1024))

        for package in data:
            match package[0]:
                case "game":
                    rows,cols = int(package[1]), int(package[2])
                    board = np.full(shape=(rows, cols),fill_value=-1)

                    player_id = package[3]

                    print("------------------- GAME START -------------------")
                    print(f'BOARD: {board}')
                    print(f'PLAYERID: {player_id}')
                    print("------------------- GAME START -------------------")

                case "tick":
                    closest_player_id = get_closest_player(other_pos)
                    #   a = atan2d(x1*y2-y1*x2,x1*x2+y1*y2);
                    #   goes from 180 to -180
                    #   thanks for formula: https://de.mathworks.com/matlabcentral/answers/180131-how-can-i-find-the-angle-between-two-vectors-including-directional-information

                    #   my vec0 = (0,1)
                    id, (x, y) = other_pos[closest_player_id]
                    x_me, y_me = own_pos

                    print(f'x.{x}, y.{y}')

                    if(x > x_me):
                        s.sendall(b"move|left\n")
                    elif (y > y_me):
                        s.sendall(b"move|down\n")
                    elif ( x < x_me):
                        s.sendall(b"move|right\n")
                    elif (y < y_me):
                        s.sendall(b"move|up\n")                   

                case "pos":
                #print(package)
                    board[int(package[3])][int(package[2])] = int(package[1]) # board x,y is == player_id
                    if(player_id == package[1]):
                        own_pos = (np.array([int(package[3]), int(package[2])]))
                        print(f'POS: {own_pos[1]} of {package[1]} !!! THIS ME !!!')
                    else:
                        other_pos.append((int(package[1]), np.array([int(package[3]), int(package[2])])))
                        print(other_pos)
                        print(f'POS: {other_pos[int(package[1])-1]} of {package[1]}')


                case "lose":
                    print("BBBBBBBBBBBBBBBBBYYYYYYYYYYYYYYYYYYYYYEEEEEEEEEEEEEEEEEE")

                case "player":
                    player_count += 1
            
            
        #this_gameloop_now(s.recv(1024))