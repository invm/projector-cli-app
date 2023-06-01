package main

import (
	"fmt"
	"log"

	"github.com/invm/projector-cli-app/go/pkg/config"
)

func main() {
	opts, err := config.GetOpts()
	if err != nil {
		log.Fatal("Unable to get opts: ", err)
	}
  fmt.Printf("opts :%+v\n", opts)
  config, err := config.NewConfig(opts)
  if err != nil {
    log.Fatal("Unable to get config: ", err)
  }
  fmt.Printf("config :%+v\n", config)
}
