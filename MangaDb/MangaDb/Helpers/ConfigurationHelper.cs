using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Serialization;
using MangaDb.Configurations;

namespace MangaDb.Helpers
{
    public static class ConfigurationHelper
    {
        private static string GetConfigFilePath(string fileName)
        {
            string path = AppSettings.MainConfigPath;
            string directory = Path.GetDirectoryName(path);

            if (!Directory.Exists(directory))
                Directory.CreateDirectory(directory);

            if (!File.Exists(path))
            {
                File.WriteAllText(path, "");
                CreateDefaultConfigurationFile(path);
            }

            return path;
        }

        public static T Deserialize<T>(string fileName)
        {
            XmlSerializer serializer = new XmlSerializer(typeof(T));
            using (FileStream file = new FileStream(GetConfigFilePath(fileName), FileMode.Open))
            {
                return (T)serializer.Deserialize(file);
            }
        }

        public static void Serialize<T>(string fileName, T obj)
        {
            XmlSerializer serializer = new XmlSerializer(typeof(T));
            using (FileStream file = new FileStream(GetConfigFilePath(fileName), FileMode.OpenOrCreate))
            {
                serializer.Serialize(file, obj);
            }
        }

        public static void CreateDefaultConfigurationFile(string path)
        {
            MainConfig conf = new MainConfig()
            {
                Entry = new Entry()
                {
                    Regex = @"<a href='(.*)' .* title='(.*: (.*))?'.*>(.*)<sup>.*>(.*)<\/a>.*rel='(.*)' .*",
                    Groups = new Groups() { Group = new List<Group>()
                    {
                        new Group() { Id = 1, PropName = "Url" },
                        new Group() { Id = 3, PropName = "Genries" },
                        new Group() { Id = 4, PropName = "Name" },
                        new Group() { Id = 5, PropName = "Translation" },
                        new Group() { Id = 6, PropName = "TumbnailUrl" }
                    }}
                },
                ListSiteUrl = @"http://grouple.ru/user/652147/bookmarks",
                RecordRegex = @"<a href='.*' class='site-element .*' .*</a>"
            };
            Serialize(path, conf);
        }

    }
}
