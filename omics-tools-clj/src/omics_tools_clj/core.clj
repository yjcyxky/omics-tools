(ns omics-tools-clj.core
  "A wrapper for several omics-tools programs"
  (:require [clojure.java.shell :as shell :refer [sh]]
            [clojure.string :as str]
            [clojure.java.io :as io]))

(defn hashmap->parameters
  "{ '-d' 'true' '-o' 'output' } -> '-d true -o output'"
  [coll]
  (str/join " " (map #(str/join " " %) (into [] coll))))

(defn extract-binary
  [bin-name]
  (let [bin-file (io/resource bin-name)
        temp-file (java.io.File/createTempFile bin-name)]
    (io/copy (io/input-stream bin-file) (io/file temp-file))
    (.getAbsolutePath bin-name)))

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
          command ["bash" "-c" (format "%s %s %s" vcf-util-cmd input-file (hashmap->parameters coll))]
          result  (apply sh command)
          status (if (= (:exit result) 0) "Success" "Error")
          msg (str (:out result) "\n" (:err result))]
      {:status status
       :msg msg})))
