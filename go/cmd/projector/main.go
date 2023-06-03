package main

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/invm/projector-cli-app/go/pkg/config"
	"github.com/invm/projector-cli-app/go/pkg/projector"
)

func main() {
	opts, err := config.GetOpts()
	if err != nil {
		log.Fatal("Unable to get opts: ", err)
	}
	cfg, err := config.NewConfig(opts)
	if err != nil {
		log.Fatal("Unable to get config: ", err)
	}
	proj := projector.NewProjector(cfg)
	if cfg.Operation == config.Print {
		if len(cfg.Args) == 0 {
			data := proj.GetValueAll()
			jsonString, err := json.Marshal(data)
			if err != nil {
				log.Fatal("This line should never be reached")
			}
			fmt.Printf("%v", string(jsonString))
		} else if value, ok := proj.GetValue(cfg.Args[0]); ok {
			fmt.Printf("%v", string(value))
		}
	}

	if cfg.Operation == config.Add {
		proj.SetValue(cfg.Args[0], cfg.Args[1])
		proj.Save()
	}

	if cfg.Operation == config.Delete {
		proj.DelValue(cfg.Args[0])
		proj.Save()
	}
}
