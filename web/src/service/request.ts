import axios from "axios";
import camelcaseKeys from "camelcase-keys";

export namespace Filebox {
  const domain = "http://localhost:8888/v1";

  export interface FileboxData {
    name: string;
    duration_day: number;
    file_type: 1 | 2; // 1=file, 2=text
    text?: string;
    file?: File;
  }

  export async function addFilebox(filebox: FileboxData) {
    const formData = new FormData();
    for (const [name, value] of Object.entries(filebox)) {
      formData.append(name, value);
    }
    const { data } = await axios.post(`${domain}/filebox`, formData, {
      headers: {
        "Content-Type": "multipart/form-data",
      },
    });
    return camelcaseKeys(data, { deep: true });
  }

  export async function getFilebox(code: string) {
    const { data } = await axios.get(`${domain}/filebox/${code}`);
    return camelcaseKeys(data, { deep: true });
  }

  export async function takeFilebox(code: string) {
    const res = await axios.post(
      `${domain}/filebox/${code}`,
      {},
      { responseType: "blob" }
    );
    fileDownload(res);
  }

  function fileDownload(res: any) {
    const blob = new Blob([res.data]);
    const elink = document.createElement("a");
    const filename = decodeURIComponent(
      // 这里使用了GB2312字符集
      res.headers["content-disposition"].replace(
        "attachment; filename*=GB2312''",
        ""
      )
    ).replace(/\"/g, "");
    elink.href = URL.createObjectURL(blob);
    elink.download = filename;
    elink.style.display = "none";
    elink.href = URL.createObjectURL(blob);
    document.body.appendChild(elink);
    elink.click();
    URL.revokeObjectURL(elink.href);
    document.body.removeChild(elink);
  }
}
