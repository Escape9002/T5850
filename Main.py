# echo-client.py

import time
import re


import socket
HOST = "localhost"  # The server's hostname or IP address
PORT = 4000  # The port used by the server

player_id = "404"
own_pos = (0,0)
rows,cols = 2,2
board = [[0 for i in range(cols)] for j in range(rows)]

def clamp_pos(value, min, max):
    if(value >= max):
        value = min
    elif (value <= min):
        value = max

    return value

def this_gameloop_now(msg):
    global player_id
    global own_pos
    global rows
    global cols
    global board

    msg = msg.decode('utf-8')
    msg = msg.split("\n")
    #print(msg)

    for package in msg:
        re.split('motd |die |pos |message |tick | lose', package)
        
        package = package.split("|")
        print(package)

        match package[0]:
            case "game":
                rows,cols = int(package[1]), int(package[2])
                board =  [[0 for i in range(cols)] for j in range(rows)]

                player_id = package[3]

                print("------------------- GAME START -------------------")
                print(f'BOARD: {board}')
                print(f'PLAYERID: {player_id}')
                print("------------------- GAME START -------------------")
                
            case "tick":
                #   x   x   x   x
                #   x   i   x   x
                #   i   o   i   x
                #   x   i   x   x
                #   x   x   x   x
                #
                # board(x,y) = val

                social_awkward = 2

                if(board[clamp_pos((own_pos[0]+social_awkward),0,len(board))] [own_pos[1]] == 0):
                    s.sendall(b"move|right\n")
                    print("----------------> RIGHT")
                elif(board[clamp_pos((own_pos[0]-social_awkward),0,len(board))] [own_pos[1]] == 0):
                    s.sendall(b"move|left\n")
                    print("----------------> LEFT")
                elif(board[own_pos[0]] [clamp_pos((own_pos[1]+social_awkward),0,len(board[0]))] == 0):
                    s.sendall(b"move|up\n")
                    print("----------------> UP")
                elif(board[own_pos[0]] [clamp_pos((own_pos[1]-social_awkward),0,len(board[0]))] == 0):
                    s.sendall(b"move|down\n")
                    print("----------------> DOWN")



            case "pos":
                #print(package)
                board[int(package[2])][int(package[3])] = int(package[1]) # board x,y is == player_id
                if(player_id == package[1]):
                    own_pos = (int(package[2]), int(package[3]))
                    print(f'POS: {own_pos} of {package[1]} !!! THIS ME !!!')
                else:
                    print(f'POS: {(int(package[2]), int(package[3]))} of {package[1]}')

            case "lose":
                print("BBBBBBBBBBBBBBBBBYYYYYYYYYYYYYYYYYYYYYEEEEEEEEEEEEEEEEEE")
            
            case default:
                return 0

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(b"join|T5850|GM965\n")

    print("connection suuuuucs")
    while(True):
        #time.sleep(500/1000) # offload the cpu a bit
        #print("pending...")
        this_gameloop_now(s.recv(1024))