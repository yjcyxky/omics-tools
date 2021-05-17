(ns omics-tools-clj.core
  "A wrapper for several omics-tools programs"
  (:require [clojure.java.shell :as shell :refer [sh]]
            [clojure.string :as str]
            [clojure.java.io :as io])
  (:import [java.nio.file.attribute FileAttribute PosixFilePermissions]
           [java.nio.file Files]))

(defn hashmap->parameters
  "{ '-d' 'true' '-o' 'output' } -> '-d true -o output'"
  [coll]
  (str/join " " (map #(str/join " " %) (into [] coll))))

(def features {:name (System/getProperty "os.name")
               :version (System/getProperty "os.version")
               :arch (System/getProperty "os.arch")})

(def resource {:vcf-util {:macosx "vcf-util-x86_64-macosx"
                          :linux "vcf-util-x86_64-linux"}})

(defn is-mac-x86-64?
  []
  (and (= (:name features) "Mac OS X")
       (= (:arch features) "x86_64")))

(defn is-linux-x86-64?
  []
  (and (= (:name features) "Linux")
       (= (:arch features) "x86_64")))

(defn get-resource-name
  [bin-name]
  (cond (is-linux-x86-64?) (:linux ((keyword bin-name) resource))
        (is-mac-x86-64?) (:macosx ((keyword bin-name) resource))))

(def tmpfile-permissions
  (into-array FileAttribute
              [(PosixFilePermissions/asFileAttribute
                (PosixFilePermissions/fromString "rwxr-xr-x"))]))

(defn extract-binary
  [bin-name]
  (let [bin-file (io/resource (get-resource-name bin-name))
        path (Files/createTempFile bin-name "" tmpfile-permissions)
        temp-file (doto (.toFile path) (.deleteOnExit))]
    (io/copy (io/input-stream bin-file) temp-file)
    (.getAbsolutePath temp-file)))

(defn call-vcf-makedb!
  "Call makedb for the vcf file.
   input-file: VCF file to process.
   output-file: Output file [default: vcf.db].
  "
  [input-file output-file]
  (shell/with-sh-env {:LC_ALL "en_US.utf-8"
                      :LANG   "en_US.utf-8"}
    (let [coll {"-o" output-file}
          vcf-util-cmd (extract-binary "vcf-util")
          command ["bash" "-c" (format "%s makedb %s %s" vcf-util-cmd input-file (hashmap->parameters coll))]
          result  (apply sh command)
          status (if (= (:exit result) 0) "Success" "Error")
          msg (str (:out result) "\n" (:err result))]
      {:status status
       :msg msg})))
