pipeline {
  agent {
    kubernetes {
      yaml '''
        apiVersion: v1
        kind: Pod
        spec:
          containers:
          - name: rust
            image: rust:latest
            command:
            - cat
            tty: true
        '''
    }
  }
  stages {
    stage('Stable') {
      parallel {
        stage('Debug') {
          steps {
            container('rust') {
              sh 'rustup toolchain install stable'
              sh 'cargo +stable build'
            }
          }
        }
        stage('Release') {
          steps {
            container('rust') {
              sh 'rustup toolchain install stable'
              sh 'cargo +stable build --release'
            }
          }
        }
      }
    }
    stage('Beta') {
      parallel {
        stage('Debug') {
          steps {
            container('rust') {
              sh 'rustup toolchain install beta'
              sh 'cargo +beta build'
            }
          }
        }
        stage('Release') {
          steps {
            container('rust') {
              sh 'rustup toolchain install beta'
              sh 'cargo +beta build --release'
            }
          }
        }
      }
    }
    stage('Nightly') {
      parallel {
        stage('Debug') {
          steps {
            container('rust') {
              sh 'rustup toolchain install nightly'
              sh 'cargo +nightly build'
            }
          }
        }
        stage('Release') {
          steps {
            container('rust') {
              sh 'rustup toolchain install nightly'
              sh 'cargo +nightly build --release'
            }
          }
        }
      }
    }
  }
}
