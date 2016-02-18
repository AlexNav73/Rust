using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using MangaDb.Configurations;
using MangaDb.Entities;
using MangaDb.Helpers;
using MangaDb.Results;

namespace MangaDb.Modules.Implementations
{
    public class ImageDownloader : IModule
    {
        public async Task<object> Execute(object context)
        {
            var res = (ParseResult) context;
            var es = res.Entries;

            HttpHelper helper = new HttpHelper();
            foreach (ListEntry entry in es)
            {
                string fileName = GetImagePath(entry.Url, entry.TumbnailUrl);

                if (!File.Exists(fileName))
                {
                    File.WriteAllBytes(fileName, await helper.DownloadImage(entry));
                }

                entry.TumbnailUrl = fileName;
            }
            return res;
        }

        private string GetImagePath(string url, string tumbnailUrl)
        {
            var reg = new Regex(@"(?=\w+$).+");
            string fileName = reg.Match(url).Value;
            string extension = reg.Match(tumbnailUrl).Value;

            fileName = string.Format("{0}.{1}", fileName, extension);

            string imageFolder = Path.Combine(Environment.CurrentDirectory, AppSettings.ImageFolder);

            if (!Directory.Exists(imageFolder))
                Directory.CreateDirectory(imageFolder);

            return Path.Combine(imageFolder, fileName);
        }
    }
}
