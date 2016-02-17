using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;

namespace MangaDb.Helpers
{
    public class HttpHelper
    {
        public async Task<string> DownloadPage(string url)
        {
            return await DownloadPageAsync(url);
        }

        private async Task<string> DownloadPageAsync(object url)
        {
            string page = (string)url;

            using (HttpClient client = new HttpClient())
                using (HttpResponseMessage response = await client.GetAsync(page))
                    using (HttpContent content = response.Content)
                    {
                        return await content.ReadAsStringAsync();
                    }
        }
    }
}
