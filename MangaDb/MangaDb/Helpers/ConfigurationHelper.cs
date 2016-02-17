using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Serialization;

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
            using (FileStream file = new FileStream(GetConfigFilePath(fileName), FileMode.Open))
            {
                serializer.Serialize(file, obj);
            }
        }

    }
}
