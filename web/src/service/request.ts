import axios from "axios";
import camelcaseKeys from "camelcase-keys";

export namespace Filebox {
  const domain = "http://localhost:8888/v1";

  export interface FileboxData {
    name: string;
    duration_day: number;
    file_type: 1 | 2; // 1=text, 2=file
    text?: string;
    file?: Blob;
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
    await axios.post(`${domain}/filebox/${code}`);
  }
}
