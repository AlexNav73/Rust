using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text;
using System.Threading.Tasks;
using MangaDb.Entities;

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

        public async Task<byte[]> DownloadImage(ListEntry item)
        {
            using (HttpClient client = new HttpClient())
                using (HttpResponseMessage response = await client.GetAsync(item.TumbnailUrl))
                    using (HttpContent content = response.Content)
                    {
                        return await content.ReadAsByteArrayAsync();
                    }
        }
    }
}
