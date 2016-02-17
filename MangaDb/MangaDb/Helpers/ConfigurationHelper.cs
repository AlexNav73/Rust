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
            return Path.Combine(Environment.CurrentDirectory, "Configs", fileName);
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

        public static void CreateDefaultConfigurationFile()
        {
            MainConfig conf = new MainConfig()
            {
                Entry = new Entry()
                {
                    Regex = @"<a href='(.*)' .* title='(.*: (.*))?'.*>(.*)<sup>.*>(.*)<\/a>.*rel='(.*)' .*",
                    Groups = new Groups() { Group = new List<Group>()
                    {
                        new Group() { Id = 0, PropName = "Name1" },
                        new Group() { Id = 1, PropName = "Name2" },
                        new Group() { Id = 2, PropName = "Name3" },
                        new Group() { Id = 3, PropName = "Name4" }
                    }}
                },
                ListSiteUrl = @"http://grouple.ru/user/652147/bookmarks",
                RecordRegex = @"<a href='.*' class='site-element .*' .*</a>"
            };
            Serialize("mainConfig.xml", conf);
        }

    }
}
