import axios from "axios";
import camelcaseKeys from "camelcase-keys";

export namespace Filebox {
  const domain = "http://localhost:8888/v1";

  export interface FileboxData {
    name: string;
    duration_day: number;
    text: string;
  }

  export async function addFilebox(filebox: FileboxData) {
    const { data } = await axios.post(`${domain}/filebox`, filebox);
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
