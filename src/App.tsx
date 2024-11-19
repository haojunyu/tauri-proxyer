import React, { useState} from "react";
import {Button, Form, Progress, Radio, RadioChangeEvent, Spin, Switch, Tooltip} from "antd"
import {LinkOutlined} from "@ant-design/icons"
import { getCurrentWindow } from "@tauri-apps/api/window"


import "./App.scss"
import { downloadExecutableFile } from "./lib/api";

enum Mode{
  BUILTIN,
  CONF,
  FILE
}

interface DownloadProgress {
  id: number;
  progress: number;
  total: number
}

interface CheckNetwork {
  tooltip: string;
  checking: boolean;
  icon: React.ReactNode;
  danger?: boolean;
  disabled?: boolean;
  style?: React.CSSProperties
}

const defaultCheckNetwork: CheckNetwork={
  tooltip: "检测代理是否有效",
  checking: false,
  icon: <LinkOutlined />,
};

const App = () =>{

  const [mode, setMode] = useState<Mode>(Mode.BUILTIN)
  const [state, setState] = useState(false);
  const [autostart, setAutostart] = useState(false);

  const [progress, setProgress] = useState<number | undefined>(undefined);
  const [switchDisable, setSwitchDisable] = useState(false);
  const [checkingNetwork, setCheckingNetwork] = useState<CheckNetwork>({...defaultCheckNetwork});

  const switchMode = (e: RadioChangeEvent) => {
    setMode(e.target.value)
  }

  const updateProgress = (progress: number, total: number) =>{
    const pgs = parseFloat(((progress/ total)*100).toFixed(1));

    setProgress(pgs);

    if(pgs ===100){
      setProgress(undefined);
    }
  };

  const download = async () => {
    setProgress(0);
    
    const unlisten = await getCurrentWindow().listen<DownloadProgress>(
      "download://progress", (e)=> {
        console.log(`trigger unlisten ${e.payload.progress}/${e.payload.total}`);
        updateProgress(e.payload.progress, e.payload.total);}
    );

    let filePath: string;
    try{
      filePath = await downloadExecutableFile();
      console.log(filePath);
      unlisten();
    }catch(e){
      setProgress(undefined);
      return;
    }
  }

  const switchProxy = async ()=> 
    await download();

  return (
    <Form id="config">
      {progress !== undefined ? (
        <div id="progress">
          <Progress type="circle" percent={progress} />
          <p>正在下载必要组件...</p>
        </div>
      ): null}



      <Form.Item label="模式">
        <Radio.Group value={mode} onChange={switchMode}>
          <Radio value={Mode.BUILTIN}>内置配置</Radio>
          <Radio value={Mode.CONF} disabled>手动配置</Radio>
          <Radio value={Mode.FILE} disabled>配置文件</Radio>
        </Radio.Group>
      </Form.Item>

      <Form.Item label="开机自启" tooltip="是否在开机时启动核心服务，此功能不影响下面的代理状态，但在代理状态开启时一起使用效果最佳">
        <Switch checked={autostart} />
      </Form.Item>

      <Form.Item label="代理状态" 
        tooltip="此处关闭代理时不会退出后台核心服务，在下次开启时速度会快一些"
        style={{display: "inline-block"}}>
        <Switch checked={state} onClick={switchProxy}
          checkedChildren="开启"
          unCheckedChildren="关闭"
          disabled={switchDisable}
        />
      </Form.Item>

      {state ? (
        <Form.Item style={{display:"inline-block", marginLeft: 20}}>
          <Spin spinning={checkingNetwork.checking}>
            <Tooltip title={checkingNetwork.tooltip}>
              <Button style={checkingNetwork.style}
                danger={checkingNetwork.danger}
                icon={checkingNetwork.icon}
                disabled={checkingNetwork.disabled}
                shape="circle"
                size="small"
                />
            </Tooltip>
          </Spin>
        </Form.Item>
      ) : null}


      <Form.Item>
        <Tooltip title="直接退出后台服务">
        <Button style={{float:"left"}}>清理后台</Button>
        </Tooltip>
      </Form.Item>


      {mode !== Mode.BUILTIN ? (
        <Form.Item>
          <Button style={{float: "right"}}>
            恢复默认设置
          </Button>
        </Form.Item>
      ): null}
    </Form>
  )

}

export default App;
