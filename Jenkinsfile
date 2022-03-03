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
        stage('Setup') {
            steps {
                container('rust') {
                    // Install latest toolchains
                    sh 'rustup toolchain install stable'
                    sh 'rustup toolchain install beta'
                    sh 'rustup toolchain install nightly'
                }
            }
        }
        stage('Stable - Check') {
            steps {
                container('rust') {
                    sh 'cargo +stable check'
                }
            }
        }
        stage('Beta - Check') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +beta check'
                    }
                }
            }
        }
        stage('Nightly - Check') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +nightly check'
                    }
                }
            }
        }
    }
}