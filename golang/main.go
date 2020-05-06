package main

import (
	"C"
	"context"
	"io"
	direct "github.com/libp2p/go-libp2p-webrtc-direct"
	ma "github.com/multiformats/go-multiaddr"
	mplex "github.com/libp2p/go-libp2p-mplex"
	tpt "github.com/libp2p/go-libp2p-core/transport"
	mux "github.com/libp2p/go-libp2p-core/mux"
	peer "github.com/libp2p/go-libp2p-core/peer"
	"github.com/pion/webrtc/v2"
)

import "C"

var transports = map[int]*direct.Transport{}
var next_transport int

var listeners = map[int]tpt.Listener{}
var next_listener int

var connections = map[int]tpt.CapableConn{}
var next_connection int

var streams = map[int]mux.MuxedStream{}
var next_stream int

//export transport_new
func transport_new() int {
	transport := direct.NewTransport(
		webrtc.Configuration{},
		new(mplex.Transport),
	)

	id := next_transport
	transports[id] = transport
	next_transport ++

	return id
}

//export transport_listen
func transport_listen(id int, multiaddr string) (int, *C.char) {
	transport := transports[id]

	maddr, err := ma.NewMultiaddr(multiaddr)

	if err != nil {
		return -1, C.CString(err.Error())
	}

	list, err := transport.Listen(maddr)

	if err != nil {
		return -1, C.CString(err.Error())
	}

	id = next_listener
	listeners[id] = list
	next_listener ++

	return id, nil
}

//export transport_dial
func transport_dial(id int, multiaddr string, peer_id string) (int, *C.char) {
	transport := transports[id]

	ctx, _ := context.WithCancel(context.Background())

	maddr, err := ma.NewMultiaddr(multiaddr)

	if err != nil {
		return -1, C.CString(err.Error())
	}

	connection, err := transport.Dial(ctx, maddr, peer.ID(peer_id))

	if err != nil {
		return -1, C.CString(err.Error())
	}

	id = next_connection
	connections[id] = connection
	next_connection ++

	return id, nil
}

//export listener_accept
func listener_accept(id int) (int, *C.char) {
	listener := listeners[id]

	connection, err := listener.Accept()

	if err != nil {
		return -1, C.CString(err.Error())
	}

	id = next_connection
	connections[id] = connection
	next_connection ++

	return id, nil
}

//export listener_close
func listener_close(id int) {
	listener := listeners[id]
	listener.Close()
}

//export connection_accept_stream
func connection_accept_stream(id int) (int, *C.char) {
	connection := connections[id]

	stream, err := connection.AcceptStream()

	if err != nil {
		return -1, C.CString(err.Error())
	}

	id = next_stream
	streams[id] = stream
	next_stream ++

	return id, nil
}

//export connection_open_stream
func connection_open_stream(id int) (int, *C.char) {
	connection := connections[id]

	stream, err := connection.OpenStream()
	if err != nil {
		return -1, C.CString(err.Error())
	}

	id = next_stream
	streams[id] = stream
	next_stream ++

	return id, nil
}

//export stream_read
func stream_read(id int, bytes []byte) (int, *C.char) {
	stream := streams[id]
	read, err := stream.Read(bytes)

	if err == io.EOF {
		return 0, nil
	}

	if err != nil {
		return -1, C.CString(err.Error())
	}

	return read, nil
}

//export stream_write
func stream_write(id int, bytes []byte) (int, *C.char) {
	stream := streams[id]
	written, err := stream.Write(bytes)

	if err != nil {
		return -1, C.CString(err.Error())
	}

	return written, nil
}

//export stream_close
func stream_close(id int) *C.char {
	stream := streams[id]
	err := stream.Close()

	if err != nil {
		return C.CString(err.Error())
	}

	return nil
}

func main() {}
