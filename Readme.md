# Network yay!
I just copy pasted my GUI to here

### TCP:
TCP - Transmission Control Protocol
- creates the connection between two devices.
- TCP is unlike simpler protocols (protocol is: ). It promises:
    - Package is recieves. If not recieved it will resend it.
        - How does it know? Through spam OK messages tillbaka i guess. 
    - In order delivery (numbered boxes)
    - uh what is a connection lifecycle...

#### practical usage:
make file manually in ur cargo:
src/bin/server.rs
src/bin/client.rs
those make it so that you can actually have a server and client. Otherwise 

### An IP:
ex, 127.0.0.1
is the adress where i'm looking for the connection
Different IP adresses have different characteristics.
127.0.0.1 is a loopback/Localhost which never leaves my device. Good for testing stuff. (Like now)

Theres: Private adresses, Public adresses
A router has a public and private adress. private is for u and no random can just access the private IP.
Public is to your router. Guessing: Which then connects to you after checking its safe?

Dynamic and static IP adressses.
Dynamic changes from time to time, (periodically), for safety i guess?
Static doesnt -> good for hosting stuff -> What we do now

### A port:
what is 
127.0.0.1:8080
8080 is the actua house number, more specific than just the adress.

IP scanning/port scanning is like for a for loop that loops through all the ports (:8080) on an IP adress.

Theres subnet scanning too, it like checks multiple IP adresses, idk what for. Security or attacks i guess?


## Things I think about when implementing
- If im black do i turn the table around? I think so
- still whites turn when beginning. Listen for move

What was the protocol again? Wait what is a protocol... Yeah protocol: Rules for how to do stuff. "Conduct procedures"
The pieces:
RNBQKBNR
PPPPPPPP
LARGE LETTERS: White
small letters: Black

When connected
the host (uh listener) sends 
W\n if host is white
B\n is host is black
also indicates started round.

loop {
    A1A3Q[char; 64]\n
    A1 - start pos
    A3 - end pos
    Q - which charcter it gest promoted to
    [char; 64] - the board, index 0 is A8 (left top)
}

so we send chars?
chat array - 


for tmr:
fix CHECKMATE and STALEMATE
they're not getting send or maybe investigare how they would be sent