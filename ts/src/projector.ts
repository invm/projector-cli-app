import { Config } from "./config";
import fs from 'fs'
import path from 'path'

export type Data = {
  projector: Record<string, Record<string, string>>
}

const defaultData = {
  projector: {}
}

export class Projector {

  constructor(private config: Config, private data: Data) {
  }

  getValueAll(): Record<string, string> {
    let curr = this.config.pwd;
    let prev = "";
    let out = {};
    const paths = [];
    do {
      prev = curr;
      paths.push(curr)
      curr = path.dirname(curr)
    } while (curr !== prev);
    out = paths.reverse().reduce((acc, path) => {
      const value = this.data.projector[path]
      if (value) {
        Object.assign(acc, value)
      }
      return acc;
    }, {})
    return out

  }

  getValue(key: string): string | undefined {
    let curr = this.config.pwd;
    let prev = "";
    let out: string = "";
    do {
      const value = this.data.projector[curr]?.[key]
      if (value) {
        out = value;
        break;
      }
      prev = curr;
      curr = path.dirname(curr)
    } while (curr !== prev);
    return out
  }

  setValue(key: string, value: string) {
    const pwd = this.config.pwd;
    if (!this.data.projector[pwd]) {
      this.data.projector[pwd] = {};
    }
    this.data.projector[pwd][key] = value;
  }

  delValue(key: string) {
    const dir = this.data.projector[this.config.pwd];
    if (dir) {
      delete dir[key]
    }
  }

  save() {
    const configPath = path.dirname(this.config.config);
    if (!fs.existsSync(configPath)) {
      fs.mkdirSync(configPath, { recursive: true })
    }
    fs.writeFileSync(this.config.config, JSON.stringify(this.data))
  }

  static fromConfig(config: Config): Projector {
    if (fs.existsSync(config.config)) {
      let data: Data;
      try {
        data = JSON.parse(fs.readFileSync(config.config).toString());
      } catch (error) {
        data = defaultData;
      }
      return new Projector(config, data)
    }
    return new Projector(config, defaultData)
  }
}
