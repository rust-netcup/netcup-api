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
        // Setup
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
        // Check
        stage('Check - Stable') {
            steps {
                container('rust') {
                    sh 'cargo +stable check'
                }
            }
        }
        stage('Check - Beta') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +beta check'
                    }
                }
            }
        }
        stage('Check - Nightly') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +nightly check'
                    }
                }
            }
        }
        // Tests
        stage('Test - Stable') {
            steps {
                container('rust') {
                    sh 'cargo +stable test'
                }
            }
        }
        stage('Test - Beta') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +beta test'
                    }
                }
            }
        }
        stage('Test - Nightly') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +nightly test'
                    }
                }
            }
        }
        // Build Debug
        stage('Build Debug - Stable') {
            steps {
                container('rust') {
                    sh 'cargo +stable build'
                }
            }
        }
        stage('Build Debug - Beta') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +beta build'
                    }
                }
            }
        }
        stage('Build Debug - Nightly') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +nightly build'
                    }
                }
            }
        }
        // Build Release
        stage('Build Release - Stable') {
            steps {
                container('rust') {
                    sh 'cargo +stable build --release'
                }
            }
        }
        stage('Build Release - Beta') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +beta build --release'
                    }
                }
            }
        }
        stage('Build Release - Nightly') {
            steps {
                container('rust') {
                    catchError(buildResult: 'SUCCESS', stageResult: 'FAILURE') {
                        sh 'cargo +nightly build --release'
                    }
                }
            }
        }
    }
}